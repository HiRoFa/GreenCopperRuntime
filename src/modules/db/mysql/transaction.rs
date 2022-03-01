use hirofa_utils::js_utils::adapters::JsRealmAdapter;
use hirofa_utils::js_utils::JsError;
use mysql_lib::Transaction;
use std::sync::{Arc, Mutex};

pub(crate) struct MysqlTransaction {
    _tx: Arc<Mutex<Transaction<'static>>>,
}

impl MysqlTransaction {
    pub(crate) fn new(tx: Transaction<'static>) -> Self {
        Self {
            _tx: Arc::new(Mutex::new(tx)),
        }
    }
    pub(crate) fn commit<R: JsRealmAdapter>(
        &self,
        realm: &R,
    ) -> Result<R::JsValueAdapterType, JsError> {
        realm.js_null_create()
    }
    /// query method
    pub fn query<R: JsRealmAdapter + 'static>(
        &self,
        _runtime: &R::JsRuntimeAdapterType,
        realm: &R,
        query: &str,
        _params: &R::JsValueAdapterType,
        _row_consumer: &R::JsValueAdapterType,
    ) -> Result<R::JsValueAdapterType, JsError> {
        log::trace!("Transaction.query: {}", query);
        /*
                let query = query.to_string();

                let mut tx = self.tx.clone();

                let rti = realm
                    .js_get_runtime_facade_inner()
                    .upgrade()
                    .expect("invalid state");

                let (params_named_vec, params_vec) = parse_params(realm, params)?;

                let row_consumer_jsvf = Arc::new(realm.to_js_value_facade(row_consumer)?);

                realm.js_promise_create_resolving_async(
                    async move {
                        log::trace!("Connection.query running async helper");
                        // in helper thread here

                        let tx_lock = &mut *tx.lock().unwrap();

                        run_query::<Conn, R>(
                            tx_lock,
                            query,
                            params_named_vec,
                            params_vec,
                            row_consumer_jsvf,
                            rti,
                        )
                        .await
                    },
                    |realm, val: Vec<JsValueFacade>| {
                        //
                        realm.from_js_value_facade(JsValueFacade::Array { val })
                    },
                )
        */
        realm.js_null_create()
    }
    pub fn execute<R: JsRealmAdapter + 'static>(
        &self,
        _runtime: &R::JsRuntimeAdapterType,
        realm: &R,
        _query: &str,
        _params_arr: &[&R::JsValueAdapterType],
    ) -> Result<R::JsValueAdapterType, JsError> {
        realm.js_null_create()
    }
    pub(crate) fn close_tx<R: JsRealmAdapter>(
        &self,
        realm: &R,
    ) -> Result<R::JsValueAdapterType, JsError> {
        realm.js_null_create()
    }
    pub(crate) fn rollback<R: JsRealmAdapter>(
        &self,
        realm: &R,
    ) -> Result<R::JsValueAdapterType, JsError> {
        realm.js_null_create()
    }
}
