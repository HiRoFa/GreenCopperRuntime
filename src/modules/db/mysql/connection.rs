use hirofa_utils::js_utils::JsError;
use mysql_lib::from_value;
use mysql_lib::prelude::Queryable;
use mysql_lib::Value;
use quickjs_runtime::esvalue::{EsNullValue, EsPromise, EsValueConvertible, EsValueFacade};
use quickjs_runtime::quickjs_utils::primitives;
use quickjs_runtime::quickjscontext::QuickJsContext;
use quickjs_runtime::valueref::JSValueRef;

pub(crate) struct MysqlConnection {
    _user: String,
    _db: String,
    _host: String,
    _port: u16,
    // todo encapsulate
    pub(crate) pool: mysql_lib::Pool,
}

impl MysqlConnection {
    pub fn new(q_ctx: &QuickJsContext, args: &[JSValueRef]) -> Result<Self, JsError> {
        // todo, actually parse args
        //url, port, user, pass, dbSchema

        let host = primitives::to_string_q(q_ctx, &args[0])?;
        let port = primitives::to_i32(&args[1])? as u16;
        let user = primitives::to_string_q(q_ctx, &args[2])?;
        let pass = primitives::to_string_q(q_ctx, &args[3])?;
        let db = primitives::to_string_q(q_ctx, &args[4])?;

        let con_str = format!("mysql://{}:{}@{}:{}/{}", user, pass, host, port, db);
        let pool =
            mysql_lib::Pool::new(con_str).map_err(|e| JsError::new_string(format!("{}", e)))?;

        Ok(Self {
            _user: user,
            _db: db,
            _host: host,
            _port: port,
            pool,
        })
    }
    /// query method
    pub fn query(
        &self,
        query: &str,
        params: EsValueFacade,
        row_consumer: EsValueFacade,
    ) -> Result<EsValueFacade, JsError> {
        // start a tx, qry, close tx
        //
        // takes three args, qry, params, consumer

        let query = query.to_string();

        let mut con = self.pool.get_conn().unwrap();

        Ok(EsPromise::new(move || {
            // in helper thread here

            // todo this should be like a future which can be awaited, this invoke_function_sync method opens doors to deadlocks
            let mut results = vec![];

            let stmt = con.prep(query).map_err(|e| format!("{}", e))?;

            let arr: Vec<Value> = if params.is_array() {
                params
                    .get_array()
                    .ok()
                    .expect("to array failed")
                    .iter()
                    .map(|esvf| {
                        if esvf.is_string() {
                            esvf.get_str().to_string().into()
                        } else if esvf.is_i32() {
                            esvf.get_i32().into()
                        } else if esvf.is_boolean() {
                            esvf.get_boolean().into()
                        } else if esvf.is_f64() {
                            esvf.get_f64().into()
                        } else {
                            // todo err? panic?
                            "".to_string().into()
                        }
                    })
                    .collect()
            } else {
                vec![]
            };

            let mut result = con.exec_iter(stmt, arr).map_err(|e| format!("{}", e))?;

            //let mut result = con.query_iter(query).map_err(|e| format!("{}", e))?;

            while let Some(result_set) = result.next_set() {
                // every row is a Vec<EsValueFacade>
                // call row consumer with that

                log::trace!("mysql::query / 1 / res_set");

                for row_res in result_set.map_err(|e| format!("{}", e))? {
                    log::trace!("mysql::query / 2 / row");

                    let mut esvf_row = vec![];

                    let row = row_res.unwrap();
                    for val_raw in row.unwrap() {
                        log::trace!("mysql::query / 3 / val");

                        match val_raw {
                            _val @ Value::NULL => {
                                esvf_row.push(EsNullValue {}.to_es_value_facade());
                            }
                            val @ Value::Bytes(..) => {
                                let i = from_value::<String>(val);
                                esvf_row.push(i.to_es_value_facade());
                            }
                            val @ Value::Int(..) => {
                                let i = from_value::<i64>(val) as i32;
                                esvf_row.push(i.to_es_value_facade());
                            }
                            val @ Value::UInt(..) => {
                                let i = from_value::<u64>(val) as i32;
                                esvf_row.push(i.to_es_value_facade());
                            }
                            val @ Value::Float(..) => {
                                let i = from_value::<f64>(val);
                                esvf_row.push(i.to_es_value_facade());
                            }
                            val @ Value::Double(..) => {
                                let i = from_value::<f64>(val);
                                esvf_row.push(i.to_es_value_facade());
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
                    let row_res_esvf = row_consumer
                        .invoke_function_sync(vec![esvf_row.to_es_value_facade()])
                        .map_err(|e| format!("{}", e))?;
                    results.push(row_res_esvf);
                }
            }

            Ok(results.to_es_value_facade())
        })
        .to_es_value_facade())
    }
    fn _start_transaction(&self) -> EsValueFacade {
        EsNullValue {}.to_es_value_facade()
    }
}
