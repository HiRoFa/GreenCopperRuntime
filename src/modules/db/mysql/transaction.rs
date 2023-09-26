use crate::modules::db::mysql::connection::parse_params;
use futures::lock::Mutex;
use mysql_lib::prelude::Queryable;
use mysql_lib::Transaction;
use quickjs_runtime::jsutils::jsproxies::JsProxyInstanceId;
use quickjs_runtime::jsutils::JsError;
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use quickjs_runtime::quickjsruntimeadapter::QuickJsRuntimeAdapter;
use quickjs_runtime::quickjsvalueadapter::QuickJsValueAdapter;
use quickjs_runtime::values::{JsValueConvertable, JsValueFacade};
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
    pub(crate) fn commit(
        &mut self,
        _runtime: &QuickJsRuntimeAdapter,
        realm: &QuickJsRealmAdapter,
        proxy_instance_id: JsProxyInstanceId,
    ) -> Result<QuickJsValueAdapter, JsError> {
        log::trace!("MysqlTransaction.commit called, setting to closed");

        let con_arc = self.tx.clone();

        self.closed = true;

        realm.create_resolving_promise_async(
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
                let _ = realm.dispatch_proxy_event(
                    &["greco", "db", "mysql"],
                    "Transaction",
                    &proxy_instance_id,
                    "commit",
                    &realm.create_null()?,
                )?;
                realm.create_null()
            },
        )
    }
    /// query method
    pub fn query(
        &self,
        _runtime: &QuickJsRuntimeAdapter,
        realm: &QuickJsRealmAdapter,
        query: &str,
        params: &QuickJsValueAdapter,
        row_consumer: &QuickJsValueAdapter,
    ) -> Result<QuickJsValueAdapter, JsError> {
        log::trace!("Transaction.query: {}", query);

        if self.closed {
            return Err(JsError::new_str("transaction is closed"));
        }

        let query = query.to_string();

        let (params_named_vec, params_vec) = parse_params(realm, params)?;

        let row_consumer_jsvf = realm.to_js_value_facade(row_consumer)?;

        // move Conn into future and get it back
        let con_arc = self.tx.clone();

        realm.create_resolving_promise_async(
            async move {
                log::trace!("MysqlTransaction.query running async helper");

                let lock_fut = con_arc.lock();
                let lock = &mut *lock_fut.await;
                let tx = lock
                    .take()
                    .ok_or_else(|| JsError::new_str("MysqlTransaction.query: invalid state"))?;

                log::trace!("MysqlTransaction.query called, tx.id={}", tx.id());

                let fut = crate::modules::db::mysql::connection::run_query::<Transaction>(
                    tx,
                    query,
                    params_named_vec,
                    params_vec,
                    row_consumer_jsvf,
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

        //realm.create_null()
    }
    pub fn execute(
        &self,
        _runtime: &QuickJsRuntimeAdapter,
        realm: &QuickJsRealmAdapter,
        query: &str,
        params_arr: &[&QuickJsValueAdapter],
    ) -> Result<QuickJsValueAdapter, JsError> {
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

        realm.create_resolving_promise_async(
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
                realm.create_f64(rows_affected as f64)
            },
        )
    }
    pub(crate) fn close_tx(
        &self,
        _runtime: &QuickJsRuntimeAdapter,
        realm: &QuickJsRealmAdapter,
    ) -> Result<QuickJsValueAdapter, JsError> {
        // todo check if committed, else rollback
        //
        // self.execute(runtime, realm, "ROLLBACK", &[])
        realm.create_null()
    }
    pub(crate) fn rollback(
        &mut self,
        _runtime: &QuickJsRuntimeAdapter,
        realm: &QuickJsRealmAdapter,
    ) -> Result<QuickJsValueAdapter, JsError> {
        if !self.closed {
            let con_arc = self.tx.clone();

            self.closed = true;

            realm.create_resolving_promise_async(
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
                move |realm, _val: ()| realm.create_null(),
            )
        } else {
            realm.create_null()
        }
    }
}

impl Drop for MysqlTransaction {
    fn drop(&mut self) {
        //let lock_fut = self.conn.lock();
        //let lock = &mut *block_on(lock_fut);
        //if let Some(mut conn) = lock.take() {
        // todo do this in helper task, which can then be async as when the connn drops it is returned to the pool
        // just be sure to spawn the task, not just create a future which is never awaited
        //let _ = block_on(conn.query_drop("ROLLBACK"));
        //}
    }
}

#[cfg(test)]
pub mod tests {

    use mysql_lib::prelude::Queryable;
    use mysql_lib::{Params, Pool, Value};

    #[tokio::test]
    async fn test() {
        let user = "hirofa";
        let pass = "hirofa";
        let host = "192.168.10.42";
        let port = 3306;
        let db = "hirofa_testdb";

        let conn_str =  format!(
            "mysql://{user}:{pass}@{host}:{port}/{db}?conn_ttl=600&stmt_cache_size=128&wait_timeout=28800"
        );

        let pool = Pool::new(conn_str.as_str());

        let mut conn = pool.get_conn().await.expect("could not connect");

        let qry = r#"
                CREATE TEMPORARY TABLE test_docs (
                `uuid`    VARCHAR(36) NOT NULL,
                `json`    MEDIUMTEXT  NOT NULL,
                PRIMARY KEY (`uuid`),
                CHECK (JSON_VALID(`json`))
            )
            ENGINE `InnoDB`
            CHARSET `utf8`
            COLLATE `utf8_general_ci`;
        "#;

        let insert_qry = r#"
            INSERT INTO test_docs(`uuid`, `json`) VALUES(?, ?)
        "#;

        let params: Vec<Value> = vec![];
        let _res = conn.exec_iter(qry, params).await.expect("could not qry");

        // insert a string with a diacrit, works fine

        let params_short_diac: Vec<Value> = vec!["abc1".into(), "{\"a\": \"André\"}".into()];

        let _res_short = conn
            .exec_iter(insert_qry, params_short_diac)
            .await
            .map_err(|e| {
                println!("err: {}", e);
                e
            })
            .expect("could not qry");

        // inset a large string without a diacrit, works fine

        let params_long_no_diac: Vec<Value> = vec!["abc2".into(), "{\"a\": \"Andre At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat.\"}".into()];

        let _res_no_diac = conn
            .exec_iter(insert_qry, params_long_no_diac)
            .await
            .map_err(|e| {
                println!("err: {}", e);
                e
            })
            .expect("could not qry");

        // inset a large string with a diacrit, fails

        let params_long_diac: Vec<Value> = vec!["abc3".into(), "{\"a\": \"André At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat.\"}".into()];

        let _res_long_diac = conn
            .exec_iter(insert_qry, params_long_diac)
            .await
            .map_err(|e| {
                println!("err: {}", e);
                e
            })
            .expect("could not qry");
    }
}
