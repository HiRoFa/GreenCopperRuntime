use hirofa_utils::js_utils::adapters::{JsRealmAdapter, JsValueAdapter};
use hirofa_utils::js_utils::facades::values::JsValueFacade;
use hirofa_utils::js_utils::JsError;
use mysql_lib::{from_value, Pool, Row};
use mysql_lib::prelude::Queryable;
use mysql_lib::Value;
use std::sync::Arc;
use futures::executor::block_on;
use cached::proc_macro::cached;

struct PoolRef {
    pool: Option<Pool>,
}

impl Drop for PoolRef {
    fn drop(&mut self) {
        // todo do this async but make sure it happens
        let _ = block_on(self.pool.take().unwrap().disconnect());
    }
}

pub struct PoolWrapper {
    arc: Arc<PoolRef>,
}

impl PoolWrapper {
    pub(crate) fn get_pool(&self) -> &Pool {
        &self.arc.pool.as_ref().unwrap()
    }
}

impl Clone for PoolWrapper {
    fn clone(&self) -> Self {
        Self {
            arc: self.arc.clone(),
        }
    }
}

pub(crate) struct MysqlConnection {
    // todo encapsulate in wrapper
    pub(crate) pool: PoolWrapper,
}

#[cached(key = "String", size = 50, time = 3600, result = true, convert = r#"{ format!("mysql://{}:p@{}:{}/{}", user, host, port, db) }"#)]
pub(crate) fn get_con_pool_wrapper(user: &str, pass: &str, host: &str, port: u16, db: &str) -> Result<PoolWrapper, JsError> {

    let con_str = format!("mysql://{}:{}@{}:{}/{}", user, pass, host, port, db);

    let pool = mysql_lib::Pool::new(con_str.as_str());

    let pw = PoolWrapper{ arc: Arc::new(PoolRef {
        pool: Some(pool)
    }) };

    Ok(pw)

}

impl MysqlConnection {
    pub fn new<R: JsRealmAdapter>(
        _runtime: &R::JsRuntimeAdapterType,
        _realm: &R,
        args: &[R::JsValueAdapterType],
    ) -> Result<Self, JsError> {
        // todo, actually parse args
        //url, port, user, pass, dbSchema

        let host = args[0].js_to_str()?;
        let port = args[1].js_to_i32() as u16;
        let user = args[2].js_to_str()?;
        let pass = args[3].js_to_str()?;
        let db = args[4].js_to_str()?;

        let pool = get_con_pool_wrapper(user, pass, host, port, db)?;

        Ok(Self {
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

        let con = self.pool.get_pool().get_conn();

        let rti = realm
            .js_get_runtime_facade_inner()
            .upgrade()
            .expect("invalid state");

        let params_jsvf = realm.to_js_value_facade(params)?;
        let row_consumer_jsvf = Arc::new(realm.to_js_value_facade(row_consumer)?);

        realm.js_promise_create_resolving_async(
            async move {
                // in helper thread here

                let mut con = con
                    .await
                    .map_err(|e| JsError::new_string(format!("{:?}", e)))?;

                let mut results: Vec<JsValueFacade> = vec![];

                //
                let stmt = con
                    .prep(query)
                    .await
                    .map_err(|e| JsError::new_string(format!("{:?}", e)))?;

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
                    .await
                    .map_err(|e| JsError::new_string(format!("{:?}", e)))?;

                //let mut result = con.query_iter(query).map_err(|e| format!("{}", e))?;

                while !result.is_empty() {
                    let result_set: Result<Vec<Row>, mysql_lib::Error> = result.collect().await;

                    // every row is a Vec<EsValueFacade>
                    // call row consumer with that

                    log::trace!("mysql::query / 1 / res_set");

                    for row_res in result_set.map_err(|e| JsError::new_string(format!("{:?}", e)))? {
                        log::trace!("mysql::query / 2 / row");

                        let mut esvf_row = vec![];

                        let row = row_res.unwrap();
                        for val_raw in row {
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
                                _val @ Value::Date(..) => {
                                    //use mysql_lib::chrono::NaiveDateTime;
                                    //println!("A date value: {}", from_value::<NaiveDateTime>(val))
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

                        if let JsValueFacade::JsFunction { cached_function } = &*row_consumer_jsvf {
                            let row_res_jsvf = cached_function.js_invoke_function(&*rti, esvf_row).await?;
                            results.push(row_res_jsvf);
                        } else {
                            panic!("row_consumer was not a function");
                        }



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
