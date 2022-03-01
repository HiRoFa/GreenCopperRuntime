use crate::modules::db::mysql::store_transaction;
use crate::modules::db::mysql::transaction::MysqlTransaction;
use cached::proc_macro::cached;
use futures::executor::block_on;
use hirofa_utils::js_utils::adapters::{JsRealmAdapter, JsRuntimeAdapter, JsValueAdapter};
use hirofa_utils::js_utils::facades::values::{JsValueFacade, TypedArrayType};
use hirofa_utils::js_utils::facades::{JsRuntimeFacade, JsValueType};
use hirofa_utils::js_utils::JsError;
use mysql_lib::consts::ColumnType;
use mysql_lib::prelude::Queryable;
use mysql_lib::{from_value, Conn, IsolationLevel, Pool, Row, TxOpts, Value};
use std::sync::Arc;

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
    pub fn get_pool(&self) -> &Pool {
        self.arc.pool.as_ref().unwrap()
    }
    pub async fn get_conn(&self) -> Result<Conn, mysql_lib::Error> {
        self.get_pool().get_conn().await
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
    pub(crate) pool: PoolWrapper,
}

#[cached(
    key = "String",
    size = 50,
    time = 3600,
    result = true,
    convert = r#"{ format!("mysql://{}:p@{}:{}/{:?}", user, host, port, db_opt) }"#
)]
pub fn get_con_pool_wrapper(
    user: &str,
    pass: &str,
    host: &str,
    port: u16,
    db_opt: Option<&str>,
) -> Result<PoolWrapper, JsError> {
    let con_str = if let Some(db) = db_opt {
        format!(
            "mysql://{}:{}@{}:{}/{}?conn_ttl=600&stmt_cache_size=128&wait_timeout=28800",
            user, pass, host, port, db
        )
    } else {
        format!(
            "mysql://{}:{}@{}:{}?conn_ttl=600&stmt_cache_size=128&wait_timeout=28800",
            user, pass, host, port
        )
    };

    let pool = mysql_lib::Pool::new(con_str.as_str());

    let pw = PoolWrapper {
        arc: Arc::new(PoolRef { pool: Some(pool) }),
    };

    Ok(pw)
}

pub(crate) async fn run_query<Q: Queryable, R: JsRealmAdapter>(
    queryable: &mut Q,
    query: String,
    params_named_vec: Option<Vec<(String, Value)>>,
    params_vec: Vec<Value>,
    row_consumer_jsvf: Arc<JsValueFacade>,
    rti: Arc<<<<R as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsRuntimeFacadeType as JsRuntimeFacade>::JsRuntimeFacadeInnerType>,
) -> Result<Vec<JsValueFacade>, JsError> {
    log::trace!("run_query: running async helper / got con");

    let mut fut_results = vec![];
    let mut results: Vec<JsValueFacade> = vec![];

    //
    let stmt = queryable
        .prep(query)
        .await
        .map_err(|e| JsError::new_string(format!("{:?}", e)))?;

    let col_types: Vec<ColumnType> = stmt.columns().iter().map(|col| col.column_type()).collect();

    log::trace!("Connection.query running async helper / prepped stmt");

    log::trace!("Connection.query running async helper / prepped params");

    let result_fut = if let Some(named_params) = params_named_vec {
        log::trace!(
            "Connection.query running async helper / prepped params / using named, size = {}",
            named_params.len()
        );
        queryable.exec_iter(stmt, named_params)
    } else {
        log::trace!(
            "Connection.query running async helper / prepped params / using positional, size = {}",
            params_vec.len()
        );
        queryable.exec_iter(stmt, params_vec)
    };

    let mut result = result_fut
        .await
        .map_err(|e| JsError::new_string(format!("{:?}", e)))?;

    log::trace!("Connection.query running async helper / got results");

    while !result.is_empty() {
        log::trace!("Connection.query running async helper / results !empty");
        let result_set: Result<Vec<Row>, mysql_lib::Error> = result.collect().await;
        log::trace!("Connection.query running async helper / got result set");
        // every row is a Vec<EsValueFacade>
        // call row consumer with that

        //let cols = result.columns().is_some()

        for row_res in result_set.map_err(|e| JsError::new_string(format!("{:?}", e)))? {
            log::trace!("mysql::query / 2 / row");

            let mut esvf_row = vec![];

            let row = row_res.unwrap();

            for (index, val_raw) in row.into_iter().enumerate() {
                log::trace!("mysql::query / 3 / val");

                match val_raw {
                    _val @ Value::NULL => {
                        esvf_row.push(JsValueFacade::Null);
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
                    val @ Value::Bytes(..) => {
                        let is_blob = col_types.len() < index
                            && col_types[index] == ColumnType::MYSQL_TYPE_BLOB;

                        if is_blob {
                            let buffer = from_value::<Vec<u8>>(val);
                            esvf_row.push(JsValueFacade::TypedArray {
                                buffer,
                                array_type: TypedArrayType::Uint8,
                            });
                        } else {
                            let i = from_value::<String>(val);
                            esvf_row.push(JsValueFacade::new_string(i));
                        }
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
                let row_res_jsvf_fut = cached_function.js_invoke_function(&*rti, esvf_row);
                fut_results.push(row_res_jsvf_fut);
            } else {
                panic!("row_consumer was not a function");
            }
        }
    }
    for row_res_jsvf_fut in fut_results {
        // normally this would be bad and you'd want to .await a join![all_futs]
        // but because these are all running in the EventLoop (and started running without us calling .await) it's ok
        results.push(row_res_jsvf_fut.await?);
    }

    Ok(results)
}

#[allow(clippy::type_complexity)]
pub(crate) fn parse_params<R: JsRealmAdapter + 'static>(
    realm: &R,
    params: &R::JsValueAdapterType,
) -> Result<(Option<Vec<(String, Value)>>, Vec<Value>), JsError> {
    let mut params_vec: Vec<Value> = vec![];
    let mut params_named_vec: Option<Vec<(String, Value)>> = None;
    log::trace!(
        "connection::parse_params params.type = {}",
        params.js_get_type()
    );
    if params.js_is_array() {
        realm.js_array_traverse_mut(params, |_index, item| {
            match item.js_get_type() {
                JsValueType::I32 => {
                    params_vec.push(item.js_to_i32().into());
                }
                JsValueType::F64 => {
                    params_vec.push(item.js_to_f64().into());
                }
                JsValueType::String => {
                    params_vec.push(item.js_to_str()?.into());
                }
                JsValueType::Boolean => {
                    params_vec.push(item.js_to_bool().into());
                }
                JsValueType::Object => {
                    if item.js_is_typed_array() {
                        let buf = realm.js_typed_array_detach_buffer(item)?;
                        params_vec.push(buf.into());
                    } else {
                        let json_str = realm.js_json_stringify(item, None)?;
                        params_vec.push(json_str.into());
                    }
                }
                JsValueType::Function => {}
                JsValueType::BigInt => {}
                JsValueType::Promise => {}
                JsValueType::Date => {}
                JsValueType::Null => {
                    params_vec.push(Value::NULL);
                }
                JsValueType::Undefined => {
                    params_vec.push(Value::NULL);
                }
                JsValueType::Array => {
                    let json_str = realm.js_json_stringify(item, None)?;
                    params_vec.push(json_str.into());
                }
                JsValueType::Error => {}
            }

            Ok(())
        })?;
    } else if params.js_is_object() {
        let mut vec = vec![];
        realm.js_object_traverse_mut(params, |name, item| {
            match item.js_get_type() {
                JsValueType::I32 => {
                    vec.push((name.to_string(), item.js_to_i32().into()));
                }
                JsValueType::F64 => {
                    vec.push((name.to_string(), item.js_to_f64().into()));
                }
                JsValueType::String => {
                    vec.push((name.to_string(), item.js_to_str()?.into()));
                }
                JsValueType::Boolean => {
                    vec.push((name.to_string(), item.js_to_bool().into()));
                }
                JsValueType::Object => {
                    if item.js_is_typed_array() {
                        let buf = realm.js_typed_array_detach_buffer(item)?;
                        vec.push((name.to_string(), buf.into()));
                    } else {
                        let json_str = realm.js_json_stringify(item, None)?;
                        vec.push((name.to_string(), json_str.into()));
                    }
                }
                JsValueType::Function => {}
                JsValueType::BigInt => {}
                JsValueType::Promise => {}
                JsValueType::Date => {}
                JsValueType::Null => {
                    vec.push((name.to_string(), Value::NULL));
                }
                JsValueType::Undefined => {
                    vec.push((name.to_string(), Value::NULL));
                }
                JsValueType::Array => {
                    let json_str = realm.js_json_stringify(item, None)?;
                    vec.push((name.to_string(), json_str.into()));
                }
                JsValueType::Error => {}
            }

            Ok(())
        })?;
        params_named_vec = Some(vec);
    }
    Ok((params_named_vec, params_vec))
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
        let db = if args[4].js_is_null_or_undefined() {
            None
        } else {
            Some(args[4].js_to_str()?)
        };

        let pool = get_con_pool_wrapper(user, pass, host, port, db)?;

        Ok(Self { pool })
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

        log::trace!("Connection.query: {}", query);

        let query = query.to_string();

        let con = self.pool.get_pool().get_conn();

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

                let mut con = con
                    .await
                    .map_err(|e| JsError::new_string(format!("{:?}", e)))?;

                run_query::<Conn, R>(
                    &mut con,
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
    }
    /// execute method
    pub fn execute<R: JsRealmAdapter + 'static>(
        &self,
        _runtime: &R::JsRuntimeAdapterType,
        realm: &R,
        query: &str,
        params_arr: &[&R::JsValueAdapterType],
    ) -> Result<R::JsValueAdapterType, JsError> {
        // start a tx, exe, close tx
        //
        // takes two args, qry, ...params

        log::trace!("Connection.execute: {}", query);

        let query = query.to_string();

        let con = self.pool.get_pool().get_conn();

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
                log::trace!("Connection.execute running async helper");
                // in helper thread here

                let mut con = con
                    .await
                    .map_err(|e| JsError::new_string(format!("{:?}", e)))?;

                log::trace!("Connection.execute running async helper / got con");

                // split queries, if multiple do in single tx?

                //let tx_opts= TxOpts::default();
                //let tx = con.start_transaction(tx_opts).await.map_err(|e| JsError::new_string(format!("{:?}", e)))?;

                //
                let stmt = con
                    .prep(query)
                    .await
                    .map_err(|e| JsError::new_string(format!("{:?}", e)))?;

                log::trace!("Connection.execute running async helper / prepped stmt");

                log::trace!("Connection.execute running async helper / prepped params");

                let result_fut = if let Some(named_params) = params_named_vec_vec {
                    log::trace!("Connection.execute running async helper / prepped params / using named, size = {}", named_params.len());
                    con.exec_batch(stmt, named_params)
                } else {
                    log::trace!("Connection.execute running async helper / prepped params / using positional, size = {}", params_vec_vec.len());
                    con.exec_batch(stmt, params_vec_vec)
                };

                result_fut
                    .await
                    .map_err(|e| JsError::new_string(format!("{:?}", e)))?;

                log::trace!("Connection.execute running async helper / got results");

                Ok(())
            },
            |realm, _val| {
                //
                realm.js_null_create()
            },
        )
    }

    pub fn start_transaction<R: JsRealmAdapter + 'static>(
        &self,
        realm: &R,
    ) -> Result<R::JsValueAdapterType, JsError> {
        let mut tx_opts = TxOpts::new();
        tx_opts.with_isolation_level(IsolationLevel::ReadCommitted);
        let pool_arc = self.pool.arc.clone();

        realm.js_promise_create_resolving_async(
            async move {
                let tx_fut = pool_arc.pool.as_ref().unwrap().start_transaction(tx_opts);
                // todo queryable wrapper for use in con and tx
                let tx = tx_fut
                    .await
                    .map_err(|err| JsError::new_string(format!("{:?}", err)))?;

                let tx_instance = MysqlTransaction::new(tx);

                Ok(tx_instance)
            },
            |realm, tx_instance| {
                let instance_id = store_transaction(tx_instance);
                realm.js_proxy_instantiate_with_id(
                    &["greco", "db", "mysql"],
                    "Transaction",
                    instance_id,
                )
            },
        )
    }
}
