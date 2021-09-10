use hirofa_utils::js_utils::adapters::{JsRealmAdapter, JsValueAdapter};
use hirofa_utils::js_utils::facades::values::JsValueFacade;
use hirofa_utils::js_utils::JsError;
use mysql_lib::from_value;
use mysql_lib::prelude::Queryable;
use mysql_lib::Opts;
use mysql_lib::Value;
use std::sync::Arc;

pub(crate) struct MysqlConnection {
    _user: String,
    _db: String,
    _host: String,
    _port: u16,
    // todo encapsulate
    pub(crate) pool: mysql_lib::Pool,
}

impl MysqlConnection {
    pub fn new<R: JsRealmAdapter>(
        _runtime: &R::JsRuntimeAdapterType,
        _realm: &R,
        args: &[R::JsValueAdapterType],
    ) -> Result<Self, JsError> {
        // todo, actually parse args
        //url, port, user, pass, dbSchema

        let host = args[0].js_to_string()?;
        let port = args[1].js_to_i32() as u16;
        let user = args[2].js_to_string()?;
        let pass = args[3].js_to_string()?;
        let db = args[4].js_to_string()?;

        let con_str = format!("mysql://{}:{}@{}:{}/{}", user, pass, host, port, db);
        let opts =
            Opts::from_url(con_str.as_str()).map_err(|e| JsError::new_string(format!("{}", e)))?;
        let pool = mysql_lib::Pool::new(opts).map_err(|e| JsError::new_string(format!("{}", e)))?;

        Ok(Self {
            _user: user,
            _db: db,
            _host: host,
            _port: port,
            pool,
        })
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
        // start a tx, qry, close tx
        //
        // takes three args, qry, params, consumer

        let query = query.to_string();

        let mut con = self.pool.get_conn().unwrap();

        let rti = realm
            .js_get_runtime_facade_inner()
            .upgrade()
            .expect("invalid state");

        let params_jsvf = realm.to_js_value_facade(params)?;
        let row_consumer_jsvf = Arc::new(realm.to_js_value_facade(row_consumer)?);

        realm.js_promise_create_resolving_async(
            async move {
                // in helper thread here

                let mut results: Vec<JsValueFacade> = vec![];

                //
                let stmt = con
                    .prep(query)
                    .map_err(|e| JsError::new_string(format!("{}", e)))?;

                let arr: Vec<Value> = match params_jsvf {
                    JsValueFacade::Array { val } => {
                        val.iter()
                            .map(|jsvf| {
                                match jsvf {
                                    JsValueFacade::I32 { val } => val.into(),
                                    JsValueFacade::F64 { val } => val.into(),
                                    JsValueFacade::String { val } => val.into(),
                                    JsValueFacade::Boolean { val } => val.into(),
                                    _ => {
                                        // todo err? panic?
                                        "".to_string().into()
                                    }
                                }
                            })
                            .collect()
                    }

                    _ => {
                        vec![]
                    }
                };

                let mut result = con
                    .exec_iter(stmt, arr)
                    .map_err(|e| JsError::new_string(format!("{}", e)))?;

                //let mut result = con.query_iter(query).map_err(|e| format!("{}", e))?;

                while let Some(result_set) = result.next_set() {
                    // every row is a Vec<EsValueFacade>
                    // call row consumer with that

                    log::trace!("mysql::query / 1 / res_set");

                    for row_res in result_set.map_err(|e| JsError::new_string(format!("{}", e)))? {
                        log::trace!("mysql::query / 2 / row");

                        let mut esvf_row = vec![];

                        let row = row_res.unwrap();
                        for val_raw in row.unwrap() {
                            log::trace!("mysql::query / 3 / val");

                            match val_raw {
                                _val @ Value::NULL => {
                                    esvf_row.push(JsValueFacade::Null);
                                }
                                val @ Value::Bytes(..) => {
                                    let i = from_value::<String>(val);
                                    esvf_row.push(JsValueFacade::new_string(i));
                                }
                                val @ Value::Int(..) => {
                                    let i = from_value::<i64>(val) as i32;
                                    esvf_row.push(JsValueFacade::new_i32(i));
                                }
                                val @ Value::UInt(..) => {
                                    let i = from_value::<u64>(val) as i32;
                                    esvf_row.push(JsValueFacade::new_i32(i));
                                }
                                val @ Value::Float(..) => {
                                    let i = from_value::<f64>(val);
                                    esvf_row.push(JsValueFacade::new_f64(i));
                                }
                                val @ Value::Double(..) => {
                                    let i = from_value::<f64>(val);
                                    esvf_row.push(JsValueFacade::new_f64(i));
                                }
                                val @ Value::Date(..) => {
                                    use mysql_lib::chrono::NaiveDateTime;
                                    println!("A date value: {}", from_value::<NaiveDateTime>(val))
                                    // todo
                                }
                                val @ Value::Time(..) => {
                                    use std::time::Duration;
                                    println!("A time value: {:?}", from_value::<Duration>(val))
                                    // todo
                                }
                            }
                        }

                        // invoke row consumer with single row data
                        // todo batch this per x rows with invoke_function_batch_sync

                        let row_res_jsva = match &*row_consumer_jsvf {
                            JsValueFacade::JsFunction { cached_function } => {
                                cached_function.js_invoke_function(&*rti, esvf_row).await?
                            }
                            _ => panic!("row_consumer was not a function"),
                        };

                        results.push(row_res_jsva);
                    }
                }
                Ok(results)
            },
            |realm, val: Vec<JsValueFacade>| {
                //
                realm.from_js_value_facade(JsValueFacade::Array { val })
            },
        )
    }
    fn _start_transaction(&self) -> JsValueFacade {
        JsValueFacade::Null
    }
}
