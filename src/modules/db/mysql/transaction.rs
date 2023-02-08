use crate::modules::db::mysql::connection::parse_params;
use futures::lock::Mutex;
use hirofa_utils::js_utils::adapters::proxies::JsProxyInstanceId;
use hirofa_utils::js_utils::adapters::JsRealmAdapter;
use hirofa_utils::js_utils::facades::values::{JsValueConvertable, JsValueFacade};
use hirofa_utils::js_utils::JsError;
use mysql_lib::prelude::Queryable;
use mysql_lib::Transaction;
use std::sync::Arc;

pub(crate) struct MysqlTransaction {
    //conn: Arc<Mutex<Option<Conn>>>,
    tx: Arc<Mutex<Option<Transaction<'static>>>>,
    closed: bool,
}

impl MysqlTransaction {
    pub(crate) fn new(tx: Transaction<'static>) -> Result<Self, JsError> {
        Ok(Self {
            // ok so this works.. but sucks... order of promise creation might not be respected if not awaiting all promises created by transaction.doSomething
            // the alternative is to block_on from the js thread.. which sucks
            // or just use a sync connection in a thread per tx... which sucks
            // or run queries async, in which case the order of promise creation might not be respected.. which sucks
            //conn: Arc::new(Mutex::new(Some(conn))),
            tx: Arc::new(Mutex::new(Some(tx))),
            closed: false,
        })
    }
    pub(crate) fn commit<R: JsRealmAdapter + 'static>(
        &mut self,
        _runtime: &R::JsRuntimeAdapterType,
        realm: &R,
        proxy_instance_id: JsProxyInstanceId,
    ) -> Result<R::JsValueAdapterType, JsError> {
        log::trace!("MysqlTransaction.commit called, setting to closed");

        let con_arc = self.tx.clone();

        self.closed = true;

        realm.js_promise_create_resolving_async(
            async move {
                log::trace!("MysqlTransaction.commit running async helper");
                let lock_fut = con_arc.lock();
                let lock = &mut *lock_fut.await;
                let tx = lock
                    .take()
                    .ok_or_else(|| JsError::new_str("MysqlTransaction.commit: invalid state"))?;

                log::trace!("MysqlTransaction.commit called, tx.id={}", tx.id());

                tx.commit()
                    .await
                    .map_err(|e| JsError::new_string(format!("{e:?}")))

                // in helper thread here
            },
            move |realm, _val: ()| {
                // dispatch commit event
                let _ = realm.js_proxy_dispatch_event(
                    &["greco", "db", "mysql"],
                    "Transaction",
                    &proxy_instance_id,
                    "commit",
                    &realm.js_null_create()?,
                )?;
                realm.js_null_create()
            },
        )
    }
    /// query method
    pub fn query<R: JsRealmAdapter + 'static>(
        &self,
        _runtime: &R::JsRuntimeAdapterType,
        realm: &R,
        query: &str,
        params: &R::JsValueAdapterType,
        row_consumer: &R::JsValueAdapterType,
    ) -> Result<R::JsValueAdapterType, JsError> {
        log::trace!("Transaction.query: {}", query);

        if self.closed {
            return Err(JsError::new_str("transaction is closed"));
        }

        let query = query.to_string();

        let rti = realm
            .js_get_runtime_facade_inner()
            .upgrade()
            .expect("invalid state");

        let (params_named_vec, params_vec) = parse_params(realm, params)?;

        let row_consumer_jsvf = realm.to_js_value_facade(row_consumer)?;

        // move Conn into future and get it back
        let con_arc = self.tx.clone();

        realm.js_promise_create_resolving_async(
            async move {
                log::trace!("MysqlTransaction.query running async helper");

                let lock_fut = con_arc.lock();
                let lock = &mut *lock_fut.await;
                let tx = lock
                    .take()
                    .ok_or_else(|| JsError::new_str("MysqlTransaction.query: invalid state"))?;

                log::trace!("MysqlTransaction.query called, tx.id={}", tx.id());

                let fut = crate::modules::db::mysql::connection::run_query::<Transaction, R>(
                    tx,
                    query,
                    params_named_vec,
                    params_vec,
                    row_consumer_jsvf,
                    rti,
                );

                let res = fut.await;

                lock.replace(res.0);

                // in helper thread here

                res.1
            },
            move |realm, val: Vec<JsValueFacade>| {
                // then

                realm.from_js_value_facade(val.to_js_value_facade())
            },
        )

        //realm.js_null_create()
    }
    pub fn execute<R: JsRealmAdapter + 'static>(
        &self,
        _runtime: &R::JsRuntimeAdapterType,
        realm: &R,
        query: &str,
        params_arr: &[&R::JsValueAdapterType],
    ) -> Result<R::JsValueAdapterType, JsError> {
        log::trace!("Transaction.execute: {}", query);

        if self.closed {
            return Err(JsError::new_str("transaction is closed"));
        }

        let query = query.to_string();

        let con_arc = self.tx.clone();

        let mut params_vec_vec = vec![];
        let mut params_named_vec_vec = None;
        for params in params_arr {
            let (params_named_vec, params_vec) = parse_params(realm, params)?;
            if let Some(named_vec) = params_named_vec {
                if params_named_vec_vec.is_none() {
                    let _ = params_named_vec_vec.replace(vec![]);
                }
                params_named_vec_vec.as_mut().unwrap().push(named_vec);
            } else {
                params_vec_vec.push(params_vec);
            }
        }

        realm.js_promise_create_resolving_async(
            async move {
                log::trace!("Transaction.execute running async helper");
                // in helper thread here

                let lock_fut = con_arc.lock();
                let lock = &mut *lock_fut.await;

                let mut tx = lock
                    .take()
                    .ok_or_else(|| JsError::new_str("MysqlTransaction.execute: invalid state"))?;

                // this blocks ensures we can reset the tx to its lock even when errors occur in execution
                let exe_res: Result<(), JsError> = async {

                    log::trace!("MysqlTransaction.execute called, tx.id={}", tx.id());

                    //
                    let stmt = tx
                        .prep(query)
                        .await
                        .map_err(|e| JsError::new_string(format!("{e:?}")))?;

                    log::trace!("Transaction.execute running async helper / prepped stmt");

                    log::trace!("Transaction.execute running async helper / prepped params");

                    let result_fut = if let Some(named_params) = params_named_vec_vec {
                        log::trace!("Transaction.execute running async helper / prepped params / using named, size = {}", named_params.len());
                        tx.exec_batch(stmt, named_params)
                    } else {
                        log::trace!("Transaction.execute running async helper / prepped params / using positional, size = {}", params_vec_vec.len());
                        tx.exec_batch(stmt, params_vec_vec)
                    };

                    result_fut
                        .await
                        .map_err(|e| JsError::new_string(format!("{e:?}")))?;

                    Ok(())

                }.await;
                let rows_affected = tx.affected_rows();
                lock.replace(tx);
                exe_res?;
                log::trace!("Transaction.execute running async helper / got results");

                Ok(rows_affected)
            },
            |realm, rows_affected| {
                //
                realm.js_f64_create(rows_affected as f64)
            },
        )
    }
    pub(crate) fn close_tx<R: JsRealmAdapter + 'static>(
        &self,
        _runtime: &R::JsRuntimeAdapterType,
        realm: &R,
    ) -> Result<R::JsValueAdapterType, JsError> {
        // todo check if committed, else rollback
        //
        // self.execute(runtime, realm, "ROLLBACK", &[])
        realm.js_null_create()
    }
    pub(crate) fn rollback<R: JsRealmAdapter + 'static>(
        &mut self,
        _runtime: &R::JsRuntimeAdapterType,
        realm: &R,
    ) -> Result<R::JsValueAdapterType, JsError> {
        if !self.closed {
            let con_arc = self.tx.clone();

            self.closed = true;

            realm.js_promise_create_resolving_async(
                async move {
                    log::trace!("MysqlTransaction.rollback running async helper");
                    let lock_fut = con_arc.lock();
                    let lock = &mut *lock_fut.await;
                    let tx = lock.take().ok_or_else(|| {
                        JsError::new_str("MysqlTransaction.rollback: invalid state")
                    })?;

                    log::trace!("MysqlTransaction.rollback called, tx.id={}", tx.id());

                    tx.rollback()
                        .await
                        .map_err(|e| JsError::new_string(format!("{e:?}")))

                    // in helper thread here
                },
                move |realm, _val: ()| realm.js_null_create(),
            )
        } else {
            realm.js_null_create()
        }
    }
}

impl Drop for MysqlTransaction {
    fn drop(&mut self) {
        //let lock_fut = self.conn.lock();
        //let lock = &mut *block_on(lock_fut);
        //if let Some(mut conn) = lock.take() {
        // todo do this in helper task, which can then be async as when the connn drops it is returned to the pool
        // just be sure to spawn the task, not just create a future whicgh is never awaited
        //let _ = block_on(conn.query_drop("ROLLBACK"));
        //}
    }
}
