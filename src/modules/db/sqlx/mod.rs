use futures::TryStreamExt;
use hirofa_utils::auto_id_map::AutoIdMap;
use jwt_simple::reexports::anyhow;
use libquickjs_sys as q;
use quickjs_runtime::builder::QuickJsRuntimeBuilder;
use quickjs_runtime::jsutils::helper_tasks::add_helper_task_async;
use quickjs_runtime::jsutils::jsproxies::JsProxy;
use quickjs_runtime::jsutils::modules::NativeModuleLoader;
use quickjs_runtime::jsutils::JsError;
use quickjs_runtime::quickjs_utils::{new_undefined, parse_args};
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use quickjs_runtime::quickjsruntimeadapter::QuickJsRuntimeAdapter;
use quickjs_runtime::quickjsvalueadapter::QuickJsValueAdapter;
use quickjs_runtime::reflection::get_proxy_instance_id;
use quickjs_runtime::values::{JsValueFacade, TypedArrayType};
use sqlx_lib::mysql::MySqlPoolOptions;
use sqlx_lib::postgres::PgPoolOptions;
use sqlx_lib::{
    Column, MySql, MySqlExecutor, PgExecutor, Pool, Postgres, Row, Transaction, TypeInfo,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};
use std::time::Duration;
use tokio::sync::RwLock;

pub enum SqlxConnection {
    PostgresConnection {
        con_str: String,
        pool: Option<Pool<Postgres>>,
    },
    MySqlConnection {
        con_str: String,
        pool: Option<Pool<MySql>>,
    },
}

pub enum SqlxTransaction {
    PostgresTransaction {
        tx: RwLock<Option<Transaction<'static, Postgres>>>,
    },
    MySqlTransaction {
        tx: RwLock<Option<Transaction<'static, MySql>>>,
    },
}

lazy_static! {
    static ref POOLS: Mutex<HashMap<String, Weak<SqlxConnection>>> = Mutex::new(HashMap::new());
}

thread_local! {
    pub static CONNECTIONS: RefCell<AutoIdMap<Arc<SqlxConnection>>> = RefCell::new(AutoIdMap::new());
    pub static TRANSACTIONS: RefCell<AutoIdMap<Arc<SqlxTransaction>>> = RefCell::new(AutoIdMap::new());
}

async fn exe_query_mysql<'e>(
    qry: String,
    args: Vec<JsValueFacade>,
    executor: impl MySqlExecutor<'e>,
    row_consumer_opt: Option<JsValueFacade>,
) -> Result<JsValueFacade, JsError> {
    let mut ret_vec: Vec<JsValueFacade> = vec![];

    let mut qry_obj = sqlx_lib::query(qry.as_str());
    for arg in args {
        // bind
        match arg {
            JsValueFacade::I32 { val } => {
                qry_obj = qry_obj.bind(val);
            }
            JsValueFacade::F64 { val } => {
                qry_obj = qry_obj.bind(val);
            }
            JsValueFacade::String { val } => {
                qry_obj = qry_obj.bind(val.to_string());
            }
            JsValueFacade::Boolean { val } => {
                qry_obj = qry_obj.bind(val);
            }
            JsValueFacade::JsObject { cached_object } => {
                let json = cached_object.to_json_string().await?;
                qry_obj = qry_obj.bind(json);
            }
            JsValueFacade::JsPromise { .. } => {}
            JsValueFacade::JsArray { .. } => {}
            JsValueFacade::JsFunction { .. } => {}
            JsValueFacade::Object { .. } => {}
            JsValueFacade::Array { .. } => {}
            JsValueFacade::Promise { .. } => {}
            JsValueFacade::Function { .. } => {}
            JsValueFacade::JsError { .. } => {}
            JsValueFacade::ProxyInstance { .. } => {}
            JsValueFacade::TypedArray { buffer, .. } => {
                qry_obj = qry_obj.bind(buffer);
            }
            JsValueFacade::JsonStr { .. } => {}
            JsValueFacade::Null => {
                qry_obj = qry_obj.bind(None::<String>);
            }
            JsValueFacade::Undefined => {
                qry_obj = qry_obj.bind(None::<String>);
            }
            _ => {
                // add null as arg
                qry_obj = qry_obj.bind(None::<String>);
            }
        }
    }
    if let Some(row_consumer) = &row_consumer_opt {
        //
        let mut rows = qry_obj.fetch(executor);

        /*
                    let mut rows = sqlx::query("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch(&mut conn);

        while let Some(row) = rows.try_next().await? {
            // map the row into a user-defined domain type
            let email: &str = row.try_get("email")?;
        }
                    */

        while let Some(row) = rows
            .try_next()
            .await
            .map_err(|e| JsError::new_string(format!("{e}")))?
        {
            //

            let mut row_args_vec: Vec<JsValueFacade> = vec![];
            for x in 0..row.len() {
                let column = row.column(x);
                let pg_type = column.type_info();

                match pg_type.name() {
                    // see https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html
                    // https://docs.rs/sqlx/latest/sqlx/mysql/types/index.html
                    "TINYINT(1)" | "BOOLEAN" | "BOOL" => {
                        let v_opt: Option<bool> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;
                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => JsValueFacade::new_bool(v),
                        };
                        row_args_vec.push(jsvf);
                    }
                    "TINYINT" | "SMALLINT" | "INT" => {
                        let v_opt: Option<i32> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;
                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => JsValueFacade::new_i32(v),
                        };
                        row_args_vec.push(jsvf);
                    }
                    "TINYINT UNSIGNED" | "SMALLINT UNSIGNED" | "INT UNSIGNED" => {
                        let v_opt: Option<u32> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;
                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => JsValueFacade::new_f64(v as f64),
                        };
                        row_args_vec.push(jsvf);
                    }
                    "BIGINT" => {
                        let v_opt: Option<i64> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;
                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => JsValueFacade::new_f64(v as f64),
                        };
                        row_args_vec.push(jsvf);
                    }
                    "BIGINT UNSIGNED" => {
                        let v_opt: Option<u64> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;
                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => JsValueFacade::new_f64(v as f64),
                        };
                        row_args_vec.push(jsvf);
                    }
                    "DECIMAL" | "FLOAT" | "DOUBLE" | "DOUBLE PRECISION" => {
                        let v_opt: Option<f64> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;
                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => JsValueFacade::new_f64(v as f64),
                        };
                        row_args_vec.push(jsvf);
                    }
                    "DATE" => {
                        let v_opt: Option<sqlx_lib::types::time::Date> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;
                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => JsValueFacade::new_string(v.to_string()),
                        };
                        row_args_vec.push(jsvf);
                    }
                    "DATETIME" => {
                        let v_opt: Option<sqlx_lib::types::time::PrimitiveDateTime> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;

                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => {
                                JsValueFacade::new_f64(v.assume_utc().unix_timestamp() as f64)
                            }
                        };
                        row_args_vec.push(jsvf);
                    }
                    "TIME" => {
                        let v_opt: Option<sqlx_lib::types::time::Time> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;

                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => JsValueFacade::new_string(v.to_string()),
                        };
                        row_args_vec.push(jsvf);
                    }
                    "VARCHAR" | "CHAR" | "ENUM" | "INET4" | "INET6" | "TEXT" | "MEDIUMTEXT"
                    | "LONGTEXT" | "LONG VARCHAR" | "ROW" | "TINYTEXT" => {
                        let v_opt: Option<String> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;
                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => JsValueFacade::new_string(v),
                        };
                        row_args_vec.push(jsvf);
                    }
                    "UUID" => {
                        let v_opt: Option<uuid::Uuid> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;

                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => {
                                JsValueFacade::new_string(v.to_string().to_ascii_uppercase())
                            }
                        };
                        row_args_vec.push(jsvf);
                    }
                    "JSON" => {
                        let v_opt: Option<serde_json::Value> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;

                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(value) => JsValueFacade::SerdeValue { value },
                        };
                        row_args_vec.push(jsvf);
                    }
                    "VARBINARY" | "BINARY" | "BLOB" => {
                        let v_opt: Option<Vec<u8>> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;
                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(buffer) => JsValueFacade::TypedArray {
                                buffer,
                                array_type: TypedArrayType::Uint8,
                            },
                        };
                        row_args_vec.push(jsvf);
                    }
                    "NULL" => {
                        row_args_vec.push(JsValueFacade::Null);
                    }
                    &_ => {
                        log::error!(
                            "COL {} TYPE {} isnull:{}",
                            column.name(),
                            pg_type.name(),
                            pg_type.is_null()
                        );
                        row_args_vec.push(JsValueFacade::Null)
                    }
                }
            }

            if let JsValueFacade::JsFunction { cached_function } = row_consumer {
                let mut func_res = cached_function.invoke_function(row_args_vec).await?;
                while let JsValueFacade::JsPromise { cached_promise } = func_res {
                    let prom_res = cached_promise.get_promise_result().await?;
                    match prom_res {
                        Ok(ok_res) => {
                            func_res = ok_res;
                        }
                        Err(rej_res) => {
                            return Err(JsError::new_string(rej_res.stringify()));
                        }
                    }
                }
                ret_vec.push(func_res);
            } else {
                return Err(JsError::new_str("row_consumer was not a function"));
            }
        }
    } else {
        let op = qry_obj
            .execute(executor)
            .await
            .map_err(|e| JsError::new_string(format!("{e}")))?;

        let mut obj: HashMap<String, JsValueFacade> = HashMap::new();
        obj.insert(
            "rowsAffected".to_string(),
            JsValueFacade::new_f64(op.rows_affected() as f64),
        );
        obj.insert(
            "lastInsertId".to_string(),
            JsValueFacade::new_f64(op.last_insert_id() as f64),
        );

        ret_vec.push(JsValueFacade::Object { val: obj });
    }

    Ok(JsValueFacade::Array { val: ret_vec })
}

async fn exe_query_postgres<'e>(
    qry: String,
    args: Vec<JsValueFacade>,
    // todo how about just pass an Either<SqlxConnection | SqlxTransaction and sort out types on fetch, hmm need to know db type for query obj>
    // DB als generic mee?
    executor: impl PgExecutor<'e>,
    row_consumer_opt: Option<JsValueFacade>,
) -> Result<JsValueFacade, JsError> {
    let mut ret_vec: Vec<JsValueFacade> = vec![];

    let mut qry_obj = sqlx_lib::query(qry.as_str());
    for arg in args {
        // bind
        match arg {
            JsValueFacade::I32 { val } => {
                qry_obj = qry_obj.bind(val);
            }
            JsValueFacade::F64 { val } => {
                qry_obj = qry_obj.bind(val);
            }
            JsValueFacade::String { val } => {
                qry_obj = qry_obj.bind(val.to_string());
            }
            JsValueFacade::Boolean { val } => {
                qry_obj = qry_obj.bind(val);
            }
            JsValueFacade::JsObject { cached_object } => {
                let json = cached_object.to_json_string().await?;
                qry_obj = qry_obj.bind(json);
            }
            JsValueFacade::JsPromise { .. } => {}
            JsValueFacade::JsArray { .. } => {}
            JsValueFacade::JsFunction { .. } => {}
            JsValueFacade::Object { .. } => {}
            JsValueFacade::Array { .. } => {}
            JsValueFacade::Promise { .. } => {}
            JsValueFacade::Function { .. } => {}
            JsValueFacade::JsError { .. } => {}
            JsValueFacade::ProxyInstance { .. } => {}
            JsValueFacade::TypedArray { buffer, .. } => {
                qry_obj = qry_obj.bind(buffer);
            }
            JsValueFacade::JsonStr { .. } => {}
            JsValueFacade::Null => {
                qry_obj = qry_obj.bind(None::<String>);
            }
            JsValueFacade::Undefined => {
                qry_obj = qry_obj.bind(None::<String>);
            }
            _ => {
                // add null as arg
                qry_obj = qry_obj.bind(None::<String>);
            }
        }
    }
    if let Some(row_consumer) = &row_consumer_opt {
        //
        let mut rows = qry_obj.fetch(executor);

        /*
                    let mut rows = sqlx::query("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch(&mut conn);

        while let Some(row) = rows.try_next().await? {
            // map the row into a user-defined domain type
            let email: &str = row.try_get("email")?;
        }
                    */

        while let Some(row) = rows
            .try_next()
            .await
            .map_err(|e| JsError::new_string(format!("{e}")))?
        {
            //

            let mut row_args_vec: Vec<JsValueFacade> = vec![];
            for x in 0..row.len() {
                let column = row.column(x);
                let pg_type = column.type_info();
                log::trace!("COL TYPE {} isnull:{}", pg_type.name(), pg_type.is_null());

                match pg_type.name() {
                    // see https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html
                    // https://docs.rs/sqlx/latest/sqlx/mysql/types/index.html
                    "BOOL" => {
                        let v_opt: Option<bool> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;
                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => JsValueFacade::new_bool(v),
                        };
                        row_args_vec.push(jsvf);
                    }
                    "SMALLINT" | "SMALLSERIAL" | "INT2" | "\"CHAR\"" | "INT" | "SERIAL"
                    | "INT4" => {
                        let v_opt: Option<i32> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;
                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => JsValueFacade::new_i32(v),
                        };
                        row_args_vec.push(jsvf);
                    }
                    "BIGINT" | "BIGSERIAL" | "INT8" => {
                        let v_opt: Option<i64> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;
                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => JsValueFacade::new_f64(v as f64),
                        };
                        row_args_vec.push(jsvf);
                    }
                    "REAL" | "FLOAT4" | "DOUBLE PRECISION" | "FLOAT8" => {
                        let v_opt: Option<f64> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;
                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => JsValueFacade::new_f64(v as f64),
                        };
                        row_args_vec.push(jsvf);
                    }
                    "DATE" => {
                        let v_opt: Option<sqlx_lib::types::time::Date> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;
                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => JsValueFacade::new_string(v.to_string()),
                        };
                        row_args_vec.push(jsvf);
                    }
                    "DATETIME" => {
                        let v_opt: Option<sqlx_lib::types::time::PrimitiveDateTime> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;

                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => {
                                JsValueFacade::new_f64(v.assume_utc().unix_timestamp() as f64)
                            }
                        };
                        row_args_vec.push(jsvf);
                    }
                    "TIME" => {
                        let v_opt: Option<sqlx_lib::types::time::Time> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;

                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => JsValueFacade::new_string(v.to_string()),
                        };
                        row_args_vec.push(jsvf);
                    }

                    "VARCHAR" | "CHAR(N)" | "TEXT" | "NAME" | "CITEXT" => {
                        let v_opt: Option<String> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;
                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => JsValueFacade::new_string(v),
                        };
                        row_args_vec.push(jsvf);
                    }
                    "UUID" => {
                        let v_opt: Option<uuid::Uuid> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;

                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(v) => {
                                JsValueFacade::new_string(v.to_string().to_ascii_uppercase())
                            }
                        };
                        row_args_vec.push(jsvf);
                    }
                    "JSON" => {
                        let v_opt: Option<serde_json::Value> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;

                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(value) => JsValueFacade::SerdeValue { value },
                        };
                        row_args_vec.push(jsvf);
                    }
                    "VARBINARY" | "BINARY" | "BLOB" | "BYTEA" => {
                        let v_opt: Option<Vec<u8>> = row
                            .try_get(x)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;
                        let jsvf = match v_opt {
                            None => JsValueFacade::Null,
                            Some(buffer) => JsValueFacade::TypedArray {
                                buffer,
                                array_type: TypedArrayType::Uint8,
                            },
                        };
                        row_args_vec.push(jsvf);
                    }
                    "NULL" => {
                        row_args_vec.push(JsValueFacade::Null);
                    }
                    &_ => {
                        log::error!(
                            "COL {} TYPE {} isnull:{}",
                            column.name(),
                            pg_type.name(),
                            pg_type.is_null()
                        );
                        row_args_vec.push(JsValueFacade::Null)
                    }
                }
            }

            if let JsValueFacade::JsFunction { cached_function } = row_consumer {
                let mut func_res = cached_function.invoke_function(row_args_vec).await?;
                while let JsValueFacade::JsPromise { cached_promise } = func_res {
                    let prom_res = cached_promise.get_promise_result().await?;
                    match prom_res {
                        Ok(ok_res) => {
                            func_res = ok_res;
                        }
                        Err(rej_res) => {
                            return Err(JsError::new_string(rej_res.stringify()));
                        }
                    }
                }
                ret_vec.push(func_res);
            } else {
                return Err(JsError::new_str("row_consumer was not a function"));
            }
        }
    } else {
        let op = qry_obj
            .execute(executor)
            .await
            .map_err(|e| JsError::new_string(format!("{e}")))?;

        let mut obj: HashMap<String, JsValueFacade> = HashMap::new();
        obj.insert(
            "rowsAffected".to_string(),
            JsValueFacade::new_f64(op.rows_affected() as f64),
        );
        obj.insert("lastInsertId".to_string(), JsValueFacade::Null);

        ret_vec.push(JsValueFacade::Object { val: obj });
    }

    Ok(JsValueFacade::Array { val: ret_vec })
}

impl Drop for SqlxConnection {
    fn drop(&mut self) {
        let map = &mut *POOLS.lock().expect("could not lock mutex");

        match self {
            SqlxConnection::PostgresConnection { con_str, pool } => {
                map.remove(con_str);
                if let Some(pool) = pool.take() {
                    let _unused = add_helper_task_async(async move {
                        pool.close().await;
                    });
                }
            }
            SqlxConnection::MySqlConnection { con_str, pool } => {
                map.remove(con_str);
                if let Some(pool) = pool.take() {
                    let _unused = add_helper_task_async(async move {
                        pool.close().await;
                    });
                }
            }
        }
    }
}

impl SqlxConnection {
    /*
        pub fn with_exe<R, C>(&self, consumer: C) -> anyhow::Result<R>
            where
                R: Sized,
                C: FnOnce(&Pool<dyn Executor>) -> R,
        {
            match self {
                SqlxConnection::PgConnection { con_str,pool } => {
                    if let Some(pool) = pool {
                        Ok(consumer(pool))
                    } else {
                        Err(anyhow!("Connection was closed"))
                    }
                }
                SqlxConnection::MysqlConnection { con_str,pool } => {
                    if let Some(pool) = pool {
                        Ok(consumer(pool))
                    } else {
                        Err(anyhow!("Connection was closed"))
                    }
                }
            }
        }
    */

    // needs to be called from inside a tokio runtime even if not async
    pub fn get_or_new(
        protocol_type: &'static str,
        host: String,
        port: u16,
        user: String,
        pass: String,
        db_opt: Option<String>,
    ) -> Result<Arc<SqlxConnection>, JsError> {
        // todo, actually parse args
        //url, port, user, pass, dbSchema
        let db = match db_opt {
            None => "".to_string(),
            Some(db_name) => {
                format!("/{db_name}")
            }
        };

        let con_str = format!("{protocol_type}://{user}:{pass}@{host}:{port}{db}");

        // see if we have a wrapper with the correct con_str
        let map = &mut *POOLS.lock().expect("could not lock mutex");
        if let Some(con_ref) = map.get(&con_str) {
            if let Some(con_arc) = con_ref.upgrade() {
                return Ok(con_arc);
            }
        }

        // todo pass options obj instead of params
        // with pool options
        let con_str2 = con_str.clone();
        let con = match protocol_type {
            "mysql" => {
                let mysql_pool = MySqlPoolOptions::new()
                    .acquire_timeout(Duration::from_secs(15))
                    .idle_timeout(Duration::from_secs(600))
                    .max_lifetime(Duration::from_secs(3600))
                    .max_connections(100)
                    .min_connections(0)
                    .connect_lazy(con_str.as_str())
                    .map_err(|e| JsError::new_string(format!("{e}")))?;
                Ok(SqlxConnection::MySqlConnection {
                    con_str,
                    pool: Some(mysql_pool),
                })
            }
            "postgres" => {
                let pg_pool = PgPoolOptions::new()
                    .acquire_timeout(Duration::from_secs(15))
                    .idle_timeout(Duration::from_secs(600))
                    .max_lifetime(Duration::from_secs(3600))
                    .max_connections(100)
                    .min_connections(0)
                    .connect_lazy(con_str.as_str())
                    .map_err(|e| JsError::new_string(format!("{e}")))?;
                Ok(SqlxConnection::PostgresConnection {
                    con_str,
                    pool: Some(pg_pool),
                })
            }
            _ => Err(JsError::new_str("unknown protocol")),
        }?;

        // register con in pools
        let arc = Arc::new(con);
        map.insert(con_str2, Arc::downgrade(&arc));
        // return arc
        Ok(arc)

        //
    }
}

async fn transaction_commit(tx: Arc<SqlxTransaction>) -> anyhow::Result<()> {
    match &*tx {
        SqlxTransaction::PostgresTransaction { tx } => {
            let tx_opt = &mut *tx.write().await;
            if let Some(tx) = tx_opt.take() {
                tx.commit().await?;
            }
        }
        SqlxTransaction::MySqlTransaction { tx } => {
            let tx_opt = &mut *tx.write().await;
            if let Some(tx) = tx_opt.take() {
                tx.commit().await?;
            }
        }
    }

    Ok(())
}

async fn transaction_rollback(tx: Arc<SqlxTransaction>) -> anyhow::Result<()> {
    match &*tx {
        SqlxTransaction::PostgresTransaction { tx } => {
            let tx_opt = &mut *tx.write().await;
            if let Some(tx) = tx_opt.take() {
                tx.rollback().await?;
            }
        }
        SqlxTransaction::MySqlTransaction { tx } => {
            let tx_opt = &mut *tx.write().await;
            if let Some(tx) = tx_opt.take() {
                tx.rollback().await?;
            }
        }
    }

    Ok(())
}

async fn transaction_close(tx: Arc<SqlxTransaction>) -> anyhow::Result<()> {
    match &*tx {
        SqlxTransaction::PostgresTransaction { tx } => {
            let tx_opt = &mut *tx.write().await;
            let _ = tx_opt.take();
        }
        SqlxTransaction::MySqlTransaction { tx } => {
            let tx_opt = &mut *tx.write().await;
            let _ = tx_opt.take();
        }
    }

    Ok(())
}

fn with_connection<R, C: FnOnce(&Arc<SqlxConnection>) -> R>(
    proxy_instance_id: usize,
    consumer: C,
) -> R {
    CONNECTIONS.with(|rc| {
        let map = &*rc.borrow();
        let con: &Arc<SqlxConnection> = map.get(&proxy_instance_id).expect("no such Connection");
        consumer(con)
    })
}

fn with_transaction<R, C: FnOnce(&Arc<SqlxTransaction>) -> R>(
    proxy_instance_id: usize,
    consumer: C,
) -> R {
    TRANSACTIONS.with(|rc| {
        let map: &AutoIdMap<Arc<SqlxTransaction>> = &rc.borrow();
        let tx_arc = map.get(&proxy_instance_id).expect("no such Transaction");
        consumer(tx_arc)
    })
}

fn store_connection(con: Arc<SqlxConnection>) -> usize {
    CONNECTIONS.with(|rc| {
        let map = &mut rc.borrow_mut();
        map.insert(con)
    })
}

fn drop_connection(proxy_instance_id: &usize) {
    CONNECTIONS.with(|rc| {
        let map = &mut rc.borrow_mut();
        map.remove(proxy_instance_id);
    })
}

fn store_transaction(tx: SqlxTransaction) -> usize {
    TRANSACTIONS.with(|rc| {
        let map = &mut *rc.borrow_mut();
        map.insert(Arc::new(tx))
    })
}

fn drop_transaction(proxy_instance_id: &usize) {
    TRANSACTIONS.with(|rc| {
        let map = &mut *rc.borrow_mut();
        map.remove(proxy_instance_id);
    })
}

struct SqlxModuleLoader {}

impl NativeModuleLoader for SqlxModuleLoader {
    fn has_module(&self, _realm: &QuickJsRealmAdapter, module_name: &str) -> bool {
        module_name.eq("greco://sqlx")
    }

    fn get_module_export_names(
        &self,
        _realm: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<&str> {
        vec![
            "Connection",
            "Transaction",
            "connectMySql",
            "connectPostgres",
        ]
    }

    fn get_module_exports(
        &self,
        realm: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<(&str, QuickJsValueAdapter)> {
        init_exports(realm).expect("init sqlx exports failed")
    }
}

pub(crate) fn init(builder: QuickJsRuntimeBuilder) -> QuickJsRuntimeBuilder {
    builder.native_module_loader(SqlxModuleLoader {})
}

fn create_connect_function(
    realm: &QuickJsRealmAdapter,
    name: &str,
    protocol: &'static str,
) -> Result<QuickJsValueAdapter, JsError> {
    realm.create_function(
        name,
        move |realm, _this, args| {
            // create promise which connects to db

            // parse args
            if !(args.len() >= 4 && args[0].is_string() && (args[1].is_i32() || args[1].is_f64() ) && args[2].is_string() && args[3].is_string() && (args.len() == 4 || args[4].is_string())) {
                return Err(JsError::new_str("connect requires 4 or 5 args (host: string, port: number, user: string, pass: string, dbName?: string)"));
            }

            let host = args[0].to_string()?;
            let port = args[1].to_i32() as u16;
            let user = args[2].to_string()?;
            let pass = args[3].to_string()?;
            let db_name_opt = if args.len() == 5 {
                Some(args[4].to_string()?)
            } else {
                None
            };

            realm.create_resolving_promise_async(
                async move {
                    // get_or_new moet aangeroepen worden in een tokio runtime.. omdat we dat nu in een async functie doen hoeven we ook niet meer connect_lazy te gebruiken maar gewoon connect
                    let con =
                        SqlxConnection::get_or_new(protocol, host, port, user, pass, db_name_opt)?;
                    Ok(con)
                },
                |realm, con| {
                    // create instance of Connection
                    let instance_id = store_connection(con);
                    realm.instantiate_proxy_with_id(
                        &["greco", "db", "sqlx"],
                        "Connection",
                        instance_id,
                    )
                },
            )
        },
        5,
    )
}

fn init_exports(
    realm: &QuickJsRealmAdapter,
) -> Result<Vec<(&'static str, QuickJsValueAdapter)>, JsError> {
    let sqlx_connection_proxy_class = create_sqlx_connection_proxy(realm);
    let sqlx_transaction_proxy_class = create_sqlx_transaction_proxy(realm);

    let con_res = realm.install_proxy(sqlx_connection_proxy_class, false)?;
    let tx_res = realm.install_proxy(sqlx_transaction_proxy_class, false)?;

    let connect_mysql = create_connect_function(realm, "connectMySql", "mysql")?;
    let connect_postgres = create_connect_function(realm, "connectPostgres", "postgres")?;

    Ok(vec![
        ("connectMySql", connect_mysql),
        ("connectPostgres", connect_postgres),
        ("Connection", con_res),
        ("Transaction", tx_res),
    ])
}

pub(crate) fn create_sqlx_transaction_proxy(_realm: &QuickJsRealmAdapter) -> JsProxy {
    JsProxy::new()
        .namespace(&["greco", "db", "sqlx"])
        .name("Transaction")
        .event_target()
        .native_method("commit", Some(fn_transaction_commit))
        .native_method("rollback", Some(fn_transaction_rollback))
        .native_method("close", Some(fn_transaction_close))
        .native_method("query", Some(fn_transaction_query))
        .native_method("execute", Some(fn_transaction_execute))
        .finalizer(|_rt, _realm, id| {
            drop_transaction(&id);
        })
}

pub(crate) fn create_sqlx_connection_proxy(_realm: &QuickJsRealmAdapter) -> JsProxy {
    JsProxy::new()
        .namespace(&["greco", "db", "sqlx"])
        .name("Connection")
        .native_method("transaction", Some(fn_connection_transaction))
        .native_method("query", Some(fn_connection_query))
        .native_method("execute", Some(fn_connection_execute))
        .finalizer(|_rt, _realm, id| {
            drop_connection(&id);
        })
}

unsafe extern "C" fn fn_connection_transaction(
    context: *mut q::JSContext,
    this_val: q::JSValue,
    _argc: ::std::os::raw::c_int,
    _argv: *mut q::JSValue,
) -> q::JSValue {
    QuickJsRuntimeAdapter::do_with(|q_js_rt| {
        log::trace!("fn_connection_transaction");

        //let args = parse_args(context, argc, argv);
        let this_val_adapter =
            QuickJsValueAdapter::new(context, this_val, true, true, "fn_commit.this");

        let q_ctx: &QuickJsRealmAdapter = q_js_rt.get_quickjs_context(context);

        if let Some(proxy_instance_id) = get_proxy_instance_id(context, &this_val_adapter) {
            // execute is called with a query and then x arrays or objects of params

            // check if first args is string and the rest are non-null objects or arrays

            // parse args into vecs of parameters

            let connection = with_connection(proxy_instance_id, |con| con.clone());

            let promise = q_ctx.create_resolving_promise_async(
                async move {
                    let con = &*connection;
                    match con {
                        SqlxConnection::PostgresConnection { pool, .. } => match pool {
                            None => Err(JsError::new_str("pool was closed")),
                            Some(pool) => {
                                let tx = pool
                                    .begin()
                                    .await
                                    .map_err(|e| JsError::new_string(format!("{e}")))?;
                                Ok(SqlxTransaction::PostgresTransaction {
                                    tx: RwLock::new(Some(tx)),
                                })
                            }
                        },
                        SqlxConnection::MySqlConnection { pool, .. } => match pool {
                            None => Err(JsError::new_str("pool was closed")),
                            Some(pool) => {
                                let tx = pool
                                    .begin()
                                    .await
                                    .map_err(|e| JsError::new_string(format!("{e}")))?;
                                Ok(SqlxTransaction::MySqlTransaction {
                                    tx: RwLock::new(Some(tx)),
                                })
                            }
                        },
                    }
                },
                |realm, res: SqlxTransaction| {
                    let instance_id = store_transaction(res);
                    realm.instantiate_proxy_with_id(
                        &["greco", "db", "sqlx"],
                        "Transaction",
                        instance_id,
                    )
                },
            );
            match promise {
                Ok(p) => p.clone_value_incr_rc(),
                Err(e) => q_ctx.report_ex(format!("could not create promise due to {e}").as_str()),
            }

            // return promise

            //// start transaction
            //// store tranaction
            //// return new instance of greco.db.sqlx.Transaction
        } else {
            q_ctx.report_ex("could not find proxy instance id")
        }
    })
}

unsafe extern "C" fn fn_connection_query(
    context: *mut q::JSContext,
    this_val: q::JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut q::JSValue,
) -> q::JSValue {
    QuickJsRuntimeAdapter::do_with(|q_js_rt| {
        log::trace!("fn_connection_query");

        //let args = parse_args(context, argc, argv);
        let this_val_adapter =
            QuickJsValueAdapter::new(context, this_val, true, true, "fn_commit.this");

        let mut args = parse_args(context, argc, argv);

        let q_ctx: &QuickJsRealmAdapter = q_js_rt.get_quickjs_context(context);

        if !(args.len() == 3
            && args[0].is_string()
            && (args[1].is_array() || args[1].is_object() || args[1].is_null())
            && args[2].is_function())
        {
            return q_ctx.report_ex("query requires three args (qry: string, arguments: Array<primitive> | Record<string, primitive>, row_consumer: () => Promise<any>)");
        }

        if let Some(proxy_instance_id) = get_proxy_instance_id(context, &this_val_adapter) {
            // execute is called with a query and then x arrays or objects of params

            // todo check if first args is string and the rest are non-null objects or arrays

            // parse args into vecs of parameters

            let connection = with_connection(proxy_instance_id, |con| con.clone());

            let row_consumer_arg = args.remove(2);

            let row_consumer_res = q_ctx.to_js_value_facade(&row_consumer_arg);
            if row_consumer_res.is_err() {
                return q_ctx.report_ex("could not extract row_consumer");
            }
            let row_consumer = row_consumer_res.ok().unwrap();

            let con_enum: &SqlxConnection = &connection;
            let protocol = match con_enum {
                SqlxConnection::PostgresConnection { .. } => Protocol::Postgres,
                SqlxConnection::MySqlConnection { .. } => Protocol::MySql,
            };

            let prepped_query_and_args_res = prep_query_and_args(q_ctx, args, protocol);

            match prepped_query_and_args_res {
                Ok(prepped_query_and_args) => {
                    let promise = q_ctx.create_resolving_promise_async(
                        async move {
                            let con_enum: &SqlxConnection = &connection;
                            match con_enum {
                                SqlxConnection::PostgresConnection { pool, .. } => {
                                    if let Some(pool) = pool {
                                        exe_query_postgres(
                                            prepped_query_and_args.0,
                                            prepped_query_and_args.1,
                                            pool,
                                            Some(row_consumer),
                                        )
                                        .await
                                    } else {
                                        Err(JsError::new_str("not connected"))
                                    }
                                }
                                SqlxConnection::MySqlConnection { pool, .. } => {
                                    if let Some(pool) = pool {
                                        exe_query_mysql(
                                            prepped_query_and_args.0,
                                            prepped_query_and_args.1,
                                            pool,
                                            Some(row_consumer),
                                        )
                                        .await
                                    } else {
                                        Err(JsError::new_str("not connected"))
                                    }
                                }
                            }
                        },
                        |realm: &QuickJsRealmAdapter, res| realm.from_js_value_facade(res),
                    );
                    match promise {
                        Ok(p) => p.clone_value_incr_rc(),
                        Err(e) => {
                            q_ctx.report_ex(format!("could not create promise due to {e}").as_str())
                        }
                    }
                }
                Err(e) => q_ctx.report_ex(
                    format!("could not parse query proxy instance for due to {e}").as_str(),
                ),
            }
        } else {
            q_ctx.report_ex("could not find proxy instance id")
        }
    })
}

lazy_static! {
    static ref PARAM_REGEX: regex::Regex = regex::Regex::new(r":\w+").unwrap();
}

enum Protocol {
    MySql,
    Postgres,
}

fn prep_query_and_args(
    realm: &QuickJsRealmAdapter,
    args: Vec<QuickJsValueAdapter>,
    protocol: Protocol,
) -> Result<(String, Vec<JsValueFacade>), JsError> {
    let query = args[0].to_str()?;

    // param names in correct order
    let mut param_names: Vec<String> = vec![];

    let mut positional_params: Vec<JsValueFacade> = Vec::new();
    let mut converted_query = String::from(query);

    for capture in PARAM_REGEX.captures_iter(query) {
        let param_name = &capture[0][1..]; // Remove leading ":"
        param_names.push(param_name.to_string());

        let replacement = match protocol {
            Protocol::MySql => "?".to_string(),
            Protocol::Postgres => format!("${}", param_names.len()),
        };

        converted_query = converted_query.replacen(&capture[0], &replacement, 1);
    }

    let arg_array_or_obj = &args[1];
    if arg_array_or_obj.is_array() {
        // convert array to vec of jsvaluefacades
        for x in 0..realm.get_array_length(arg_array_or_obj)? {
            let element = realm.get_array_element(arg_array_or_obj, x)?;
            positional_params.push(realm.to_js_value_facade(&element)?);
        }
    } else if arg_array_or_obj.is_object() && !arg_array_or_obj.is_null() {
        // convert obj to vec of jsvaluefacades in order of param_names
        for param_name in &param_names {
            let element = realm.get_object_property(arg_array_or_obj, param_name)?;
            //if element.is_typed_array() {
            //    positional_params.push(JsValueFacade::Null);
            //} else {
            positional_params.push(realm.to_js_value_facade(&element)?);
            //}
        }
    } else if arg_array_or_obj.is_null() {
        // no args
    } else {
        return Err(JsError::new_str("argument was not an array or object"));
    }

    Ok((converted_query, positional_params))
}

unsafe extern "C" fn fn_connection_execute(
    context: *mut q::JSContext,
    this_val: q::JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut q::JSValue,
) -> q::JSValue {
    QuickJsRuntimeAdapter::do_with(|q_js_rt| {
        log::trace!("fn_connection_execute");

        //let args = parse_args(context, argc, argv);
        let this_val_adapter =
            QuickJsValueAdapter::new(context, this_val, true, true, "fn_commit.this");

        let args = parse_args(context, argc, argv);

        let q_ctx: &QuickJsRealmAdapter = q_js_rt.get_quickjs_context(context);

        if !(args.len() == 2
            && args[0].is_string()
            && (args[1].is_object() || args[1].is_array() || args[1].is_null()))
        {
            return q_ctx.report_ex("execute requires two or more args (qry: string, arguments1: Array<primitive> | Record<string, primitive>, arguments2?: Array<primitive> | Record<string, primitive>, arguments3?: Array<primitive> | Record<string, primitive>, etc)");
        }

        if let Some(proxy_instance_id) = get_proxy_instance_id(context, &this_val_adapter) {
            // execute is called with a query and then x arrays or objects of params

            // check if first args is string and the rest are non-null objects or arrays

            // parse args into vecs of parameters

            let connection = with_connection(proxy_instance_id, |con| con.clone());

            let con_enum: &SqlxConnection = &connection;
            let protocol = match con_enum {
                SqlxConnection::PostgresConnection { .. } => Protocol::Postgres,
                SqlxConnection::MySqlConnection { .. } => Protocol::MySql,
            };

            let prepped_query_and_args_res = prep_query_and_args(q_ctx, args, protocol);
            match prepped_query_and_args_res {
                Ok(prepped_query_and_args) => {
                    let promise = q_ctx.create_resolving_promise_async(
                        async move {
                            let con_enum: &SqlxConnection = &connection;
                            match con_enum {
                                SqlxConnection::PostgresConnection { pool, .. } => {
                                    if let Some(pool) = pool {
                                        exe_query_postgres(
                                            prepped_query_and_args.0,
                                            prepped_query_and_args.1,
                                            pool,
                                            None,
                                        )
                                        .await
                                    } else {
                                        Err(JsError::new_str("not connected"))
                                    }
                                }
                                SqlxConnection::MySqlConnection { pool, .. } => {
                                    if let Some(pool) = pool {
                                        exe_query_mysql(
                                            prepped_query_and_args.0,
                                            prepped_query_and_args.1,
                                            pool,
                                            None,
                                        )
                                        .await
                                    } else {
                                        Err(JsError::new_str("not connected"))
                                    }
                                }
                            }
                        },
                        |realm: &QuickJsRealmAdapter, res| realm.from_js_value_facade(res),
                    );
                    match promise {
                        Ok(p) => p.clone_value_incr_rc(),
                        Err(e) => q_ctx
                            .report_ex(format!("could not create promise due to {e}",).as_str()),
                    }
                }
                Err(e) => q_ctx.report_ex(
                    format!("could not parse query proxy instance for due to {e}").as_str(),
                ),
            }
        } else {
            q_ctx.report_ex("could not find proxy instance id")
        }
    })
}

unsafe extern "C" fn fn_transaction_query(
    context: *mut q::JSContext,
    this_val: q::JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut q::JSValue,
) -> q::JSValue {
    QuickJsRuntimeAdapter::do_with(|q_js_rt| {
        log::trace!("fn_transaction_query");

        let this_val_adapter =
            QuickJsValueAdapter::new(context, this_val, true, true, "fn_commit.this");

        let mut args = parse_args(context, argc, argv);

        let q_ctx: &QuickJsRealmAdapter = q_js_rt.get_quickjs_context(context);

        if !(args.len() == 3
            && args[0].is_string()
            && (args[1].is_array() || args[1].is_object() || args[1].is_null())
            && args[2].is_function())
        {
            return q_ctx.report_ex("query requires three args (qry: string, arguments: Array<primitive> | Record<string, primitive>, row_consumer: () => Promise<any>)");
        }

        if let Some(proxy_instance_id) = get_proxy_instance_id(context, &this_val_adapter) {
            // execute is called with a query and then x arrays or objects of params

            // todo check if first args is string and the rest are non-null objects or arrays

            // parse args into vecs of parameters

            let transaction = with_transaction(proxy_instance_id, |tx| tx.clone());
            let protocol = {
                match &*transaction {
                    SqlxTransaction::PostgresTransaction { .. } => Protocol::Postgres,
                    SqlxTransaction::MySqlTransaction { .. } => Protocol::MySql,
                }
            };
            let row_consumer_arg = args.remove(2);

            let row_consumer_res = q_ctx.to_js_value_facade(&row_consumer_arg);
            if row_consumer_res.is_err() {
                return q_ctx.report_ex("could not extract row_consumer");
            }
            let row_consumer = row_consumer_res.ok().unwrap();

            // todo rewrite prep function so i can obtain protocol after write lock of tx
            // so split into prep_query and parse args
            let prepped_query_and_args_res = prep_query_and_args(q_ctx, args, protocol);

            match prepped_query_and_args_res {
                Ok(prepped_query_and_args) => {
                    let promise = q_ctx.create_resolving_promise_async(
                        async move {
                            match &*transaction {
                                SqlxTransaction::PostgresTransaction { tx, .. } => {
                                    let write_locked = &mut *tx.write().await;
                                    if let Some(tx) = write_locked {
                                        let exe = &mut **tx;
                                        exe_query_postgres(
                                            prepped_query_and_args.0,
                                            prepped_query_and_args.1,
                                            exe,
                                            Some(row_consumer),
                                        )
                                        .await
                                    } else {
                                        Err(JsError::new_str("Transaction was closed"))
                                    }
                                }
                                SqlxTransaction::MySqlTransaction { tx, .. } => {
                                    let write_locked = &mut *tx.write().await;
                                    if let Some(tx) = write_locked {
                                        let exe = &mut **tx;
                                        exe_query_mysql(
                                            prepped_query_and_args.0,
                                            prepped_query_and_args.1,
                                            exe,
                                            Some(row_consumer),
                                        )
                                        .await
                                    } else {
                                        Err(JsError::new_str("Transaction was closed"))
                                    }
                                }
                            }
                        },
                        |realm: &QuickJsRealmAdapter, res| realm.from_js_value_facade(res),
                    );
                    match promise {
                        Ok(p) => p.clone_value_incr_rc(),
                        Err(e) => {
                            q_ctx.report_ex(format!("could not create promise due to {e}").as_str())
                        }
                    }
                }
                Err(e) => q_ctx.report_ex(
                    format!("could not parse query proxy instance for due to {e}").as_str(),
                ),
            }
        } else {
            q_ctx.report_ex("could not find proxy instance id")
        }
    })
}

unsafe extern "C" fn fn_transaction_close(
    context: *mut q::JSContext,
    this_val: q::JSValue,
    _argc: ::std::os::raw::c_int,
    _argv: *mut q::JSValue,
) -> q::JSValue {
    QuickJsRuntimeAdapter::do_with(|q_js_rt| {
        log::trace!("transaction close");

        //let args = parse_args(context, argc, argv);
        let this_val_adapter =
            QuickJsValueAdapter::new(context, this_val, true, true, "fn_transaction_close.this");

        let q_ctx = q_js_rt.get_quickjs_context(context);

        if let Some(proxy_instance_id) = get_proxy_instance_id(context, &this_val_adapter) {
            // return a promise which async closes the connection

            let transaction = with_transaction(proxy_instance_id, |tx| tx.clone());

            let promise = q_ctx.create_resolving_promise_async(
                async move {
                    // produce
                    transaction_close(transaction)
                        .await
                        .map_err(|e| JsError::new_string(format!("{e}")))
                },
                move |realm, _res| {
                    let _ = realm.dispatch_proxy_event(
                        &["greco", "db", "sqlx"],
                        "Transaction",
                        &proxy_instance_id,
                        "close",
                        &realm.create_null()?,
                    )?;
                    // map
                    realm.create_undefined()
                },
            );
            match promise {
                Ok(p) => p.clone_value_incr_rc(),
                Err(_) => {
                    // todo report error?
                    new_undefined()
                }
            }
        } else {
            // todo report error?
            new_undefined()
        }
    })
}

unsafe extern "C" fn fn_transaction_rollback(
    context: *mut q::JSContext,
    this_val: q::JSValue,
    _argc: ::std::os::raw::c_int,
    _argv: *mut q::JSValue,
) -> q::JSValue {
    QuickJsRuntimeAdapter::do_with(|q_js_rt| {
        log::trace!("transaction rollback");

        //let args = parse_args(context, argc, argv);
        let this_val_adapter = QuickJsValueAdapter::new(
            context,
            this_val,
            true,
            true,
            "fn_transaction_rollback.this",
        );

        let q_ctx = q_js_rt.get_quickjs_context(context);

        if let Some(proxy_instance_id) = get_proxy_instance_id(context, &this_val_adapter) {
            // return a promise which async closes the connection

            let transaction = with_transaction(proxy_instance_id, |tx| tx.clone());

            let promise = q_ctx.create_resolving_promise_async(
                async move {
                    // produce
                    transaction_rollback(transaction)
                        .await
                        .map_err(|e| JsError::new_string(format!("{e}")))
                },
                move |realm, _res| {
                    let _ = realm.dispatch_proxy_event(
                        &["greco", "db", "sqlx"],
                        "Transaction",
                        &proxy_instance_id,
                        "rollback",
                        &realm.create_null()?,
                    )?;
                    // map
                    realm.create_undefined()
                },
            );
            match promise {
                Ok(p) => p.clone_value_incr_rc(),
                Err(_) => {
                    // todo report error?
                    new_undefined()
                }
            }
        } else {
            // todo report error?
            new_undefined()
        }
    })
}

unsafe extern "C" fn fn_transaction_execute(
    context: *mut q::JSContext,
    this_val: q::JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut q::JSValue,
) -> q::JSValue {
    QuickJsRuntimeAdapter::do_with(|q_js_rt| {
        log::trace!("fn_transaction_execute");

        //let args = parse_args(context, argc, argv);
        let this_val_adapter =
            QuickJsValueAdapter::new(context, this_val, true, true, "fn_commit.this");

        let args = parse_args(context, argc, argv);

        let q_ctx: &QuickJsRealmAdapter = q_js_rt.get_quickjs_context(context);

        if !(args.len() == 2
            && args[0].is_string()
            && (args[1].is_object() || args[1].is_array() || args[1].is_null()))
        {
            return q_ctx.report_ex("execute requires two or more args (qry: string, arguments1: Array<primitive> | Record<string, primitive>, arguments2?: Array<primitive> | Record<string, primitive>, arguments3?: Array<primitive> | Record<string, primitive>, etc)");
        }

        if let Some(proxy_instance_id) = get_proxy_instance_id(context, &this_val_adapter) {
            // execute is called with a query and then x arrays or objects of params

            // check if first args is string and the rest are non-null objects or arrays

            // parse args into vecs of parameters

            let transaction = with_transaction(proxy_instance_id, |tx| tx.clone());

            let protocol = match &*transaction {
                SqlxTransaction::PostgresTransaction { .. } => Protocol::Postgres,
                SqlxTransaction::MySqlTransaction { .. } => Protocol::MySql,
            };

            let prepped_query_and_args_res = prep_query_and_args(q_ctx, args, protocol);
            match prepped_query_and_args_res {
                Ok(prepped_query_and_args) => {
                    let promise = q_ctx.create_resolving_promise_async(
                        async move {
                            match &*transaction {
                                SqlxTransaction::PostgresTransaction { tx, .. } => {
                                    let write_locked = &mut *tx.write().await;
                                    if let Some(tx) = write_locked {
                                        let exe = &mut **tx;
                                        exe_query_postgres(
                                            prepped_query_and_args.0,
                                            prepped_query_and_args.1,
                                            exe,
                                            None,
                                        )
                                        .await
                                    } else {
                                        Err(JsError::new_str("Transaction was closed"))
                                    }
                                }
                                SqlxTransaction::MySqlTransaction { tx, .. } => {
                                    let write_locked = &mut *tx.write().await;
                                    if let Some(tx) = write_locked {
                                        let exe = &mut **tx;
                                        exe_query_mysql(
                                            prepped_query_and_args.0,
                                            prepped_query_and_args.1,
                                            exe,
                                            None,
                                        )
                                        .await
                                    } else {
                                        Err(JsError::new_str("Transaction was closed"))
                                    }
                                }
                            }
                        },
                        |realm: &QuickJsRealmAdapter, res| realm.from_js_value_facade(res),
                    );
                    match promise {
                        Ok(p) => p.clone_value_incr_rc(),
                        Err(e) => q_ctx
                            .report_ex(format!("could not create promise due to {e}",).as_str()),
                    }
                }
                Err(e) => q_ctx.report_ex(
                    format!("could not parse query proxy instance for due to {e}").as_str(),
                ),
            }
        } else {
            q_ctx.report_ex("could not find proxy instance id")
        }
    })
}

unsafe extern "C" fn fn_transaction_commit(
    context: *mut q::JSContext,
    this_val: q::JSValue,
    _argc: ::std::os::raw::c_int,
    _argv: *mut q::JSValue,
) -> q::JSValue {
    QuickJsRuntimeAdapter::do_with(|q_js_rt| {
        log::trace!("transaction commit");

        //let args = parse_args(context, argc, argv);
        let this_val_adapter =
            QuickJsValueAdapter::new(context, this_val, true, true, "fn_transaction_commit.this");

        let q_ctx = q_js_rt.get_quickjs_context(context);

        if let Some(proxy_instance_id) = get_proxy_instance_id(context, &this_val_adapter) {
            // return a promise which async closes the connection

            let transaction = with_transaction(proxy_instance_id, |tx| tx.clone());

            let promise = q_ctx.create_resolving_promise_async(
                async move {
                    // produce
                    transaction_commit(transaction)
                        .await
                        .map_err(|e| JsError::new_string(format!("{e}")))
                },
                move |realm, _res| {
                    let _ = realm.dispatch_proxy_event(
                        &["greco", "db", "sqlx"],
                        "Transaction",
                        &proxy_instance_id,
                        "commit",
                        &realm.create_null()?,
                    )?;

                    // map
                    realm.create_undefined()
                },
            );
            match promise {
                Ok(p) => p.clone_value_incr_rc(),
                Err(_) => {
                    // todo report error?
                    new_undefined()
                }
            }
        } else {
            // todo report error?
            new_undefined()
        }
    })
}

#[cfg(test)]
pub mod tests {
    use backtrace::Backtrace;
    use std::panic;
    //use log::LevelFilter;
    use quickjs_runtime::builder::QuickJsRuntimeBuilder;
    use quickjs_runtime::jsutils::Script;
    use quickjs_runtime::values::JsValueFacade;

    #[tokio::test]
    async fn _test_sqlx() {
        panic::set_hook(Box::new(|panic_info| {
            let backtrace = Backtrace::new();
            log::error!(
                "thread panic occurred: {}\nbacktrace: {:?}",
                panic_info,
                backtrace
            );
        }));

        //simple_logging::log_to_file("grecort.log", LevelFilter::Info)
        //    .ok()
        //    .expect("could not init logger");

        //simple_logging::log_to_stderr(log::LevelFilter::Info);

        let builder = QuickJsRuntimeBuilder::new();
        let builder = crate::init_greco_rt(builder);
        let rt = builder.build();

        let script = Script::new(
            "test_mysql.js",
            r#"

        async function testPg() {
            let sqlxMod = await import('greco://sqlx');
            //let host = '127.0.0.1';
            let host = '192.168.10.43';
            let port = 5432;
            let user = 'hirofa';
            let pass = 'hirofa';
            let db = 'hirofa_testdb';
            let con = await sqlxMod.connectPostgres(host, port, user, pass, db);

            console.log("Pg Connected");
            
            await con.execute(`DROP TABLE IF EXISTS test`, []);
            
            console.log("Pg Executed drop if exists");
            
            await con.execute(`
                CREATE TABLE test(
                    "id" SERIAL PRIMARY KEY,
                    "test" VARCHAR(32),
                    "uuid" UUID,
                    "when" DATE,
                    "json" JSON,
                    "blob" BYTEA,
                    "text" TEXT
                )
            `, []);
            
            console.log("Pg Executed create table");
            
            const obj1 = {hello: "world1"};
            
            await con.execute(`
                INSERT into test("test", "uuid", "when", "json", "text") VALUES('hi1', '0001-0002-00C0-A0000000-F00000000001', CURRENT_DATE, '{"a": 1}', 'lorem ipsum1')
            `, []);

            console.log("Pg Executed insert into 1");

      
            await con.execute(`
                INSERT into test("test", "uuid", "when", "json", "text") VALUES('hi2', '0000-0000-0000-00000000-000000000002', CURRENT_DATE, '{"hello": "world1"}', 'lorem ipsum2')
            `, []);


            console.log("Pg Executed insert into 2");

            await con.execute(`
                INSERT into test("test", "uuid", "when", "json", "text") VALUES($1, '0000-0000-0000-00000000-000000000002', CURRENT_DATE, '{"hello":"world2"}', 'lorem ipsum2')
            `, ['hi3']);

            console.log("Pg Executed insert into 3");

            await con.execute(`
                INSERT into test("test", "uuid", "when", "json", "text") VALUES($1, '0000-0000-0000-00000000-000000000003', CURRENT_DATE, to_json($2), 'lorem ipsum3')
            `, ['hi4', obj1]);

            console.log("Pg Executed insert into 4");

            await con.query('select * from test where "test" = $1', ['hi1'], (...row) => {
                for (let x = 0; x < row.length; x++) {
                    console.log('Pg col %s = %o (typeof = %s)', x, row[x], typeof row[x]);
                }
            });

            console.log("Pg Executed select 1");

            await con.query('select * from test where "test" = :a', {a: 'hi2'}, (...row) => {
                for (let x = 0; x < row.length; x++) {
                    console.log('Pg named col %s = %o', x, row[x]);
                }
            });

            console.log("Pg Executed select 2");

            await con.query('select * from test', null, (...row) => {
                for (let x = 0; x < row.length; x++) {
                    console.log('Pg noparams col %s = %o', x, row[x]);
                }
            });

            let ct = await con.execute('delete from test', []);
            console.log('Pg deleted rows %o', ct);
            ct = await con.execute('delete from test', []);
            console.log('Pg deleted rows %o', ct);

            await con.execute('insert into test("id") values($1)', [4]);

            let tables = await con.query(`SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'`, null, (row1) => {
                return row1;
            });

            for (let table of tables) {
                console.log('Pg found table %s', table);
            }
            
            console.log('Starting Pg tx');
            for (let x = 0; x < 10; x++) {
                let tx = await con.transaction();
                try  {
                    let res = await tx.query('SELECT 123' + x, null, async (...row) => {
                        return row[0];
                    });            
                    console.log('Pg tx res = %o', res);
                    await tx.commit();
                } catch(ex) {
                    console.log("Pg fail: %s", ex);
                    console.error(ex);
                    try {
                        await tx.rollback();
                    } catch(ex) {
                        // care
                    }
                } finally {
                    await tx.close();
                }
            }
            
            console.log('Pg tx done');

        }

        async function testMySql() {
            let sqlxMod = await import('greco://sqlx');
            //let host = '127.0.0.1';
            let host = '192.168.10.43';
            let port = 3306;
            let user = 'hirofa';
            let pass = 'hirofa';
            let db = 'hirofa_testdb';
            let con = await sqlxMod.connectMySql(host, port, user, pass, db);

            console.log("Connected");

            await con.execute(`DROP TABLE IF EXISTS test`, []);
            
            console.log("Executed drop if exists");
            
            await con.execute(`
                CREATE TABLE test(
                    \`id\` INT auto_increment PRIMARY KEY,
                    \`test\` VARCHAR(32),
                    \`uuid\` UUID,
                    \`when\` DATE,
                    \`json\` JSON,
                    \`blob\` LONGBLOB,
                    \`text\` LONGTEXT
                )
            `, []);
            
            console.log("Executed create table");
            
            const obj1 = {hello: "world1"};
            
            
            
            await con.execute(`
                INSERT into test(\`test\`, \`uuid\`, \`when\`, \`json\`, \`text\`) VALUES('hi0', '0001-0002-00C0-A0000000-F00000000001', CURDATE(), ?, 'lorem ipsum1')
            `, [obj1]);
            
       
            await con.execute(`
                INSERT into test(\`test\`, \`uuid\`, \`when\`, \`json\`, \`text\`) VALUES('hi2', '0000-0000-0000-00000000-000000000002', CURDATE(), '{}', 'lorem ipsum2')
            `, []);
            await con.execute(`
                INSERT into test(\`test\`, \`uuid\`, \`when\`, \`json\`, \`text\`) VALUES('hi2', '0000-0000-0000-00000000-000000000003', CURDATE(), '{}', 'lorem ipsum3')
            `, []);
            
            
            ////show indexes FROM test
            await con.query(`show indexes FROM test`, [], (...row) => {
                for (let x = 0; x < row.length; x++) {
                    console.log('idx col %s = %s', x, row[x]);
                }
            });

           
            await con.query('select * from test where \`test\` = :a', {a: 'hi2'}, (...row) => {
                for (let x = 0; x < row.length; x++) {
                    console.log('named col %s = %s', x, row[x]);
                }
            });

            await con.query('select * from test', null, (...row) => {
                for (let x = 0; x < row.length; x++) {
                    console.log('noparams col %s = %s', x, row[x]);
                }
            });

            let ct = await con.execute('delete from test', []);
            console.log('deleted rows %o', ct);
            ct = await con.execute('delete from test', []);
            console.log('deleted rows %o', ct);

            await con.execute('insert into test(id) values(?)', [4]);

            let tables = await con.query('show tables', null, (row1) => {
                return row1;
            });

            for (let table of tables) {
                console.log('found table %s', table);
            }
            
            console.log('Starting MySql tx');
            
            let grecoDom = await import("greco://htmldom");
            let ParserClass = grecoDom.DOMParser;
            let parser = new ParserClass();
            let ps = [];
            for (let x = 0; x < 500000; x++) {
                ps.push("<p>hello world</p>");
            }
            let htmlDoc = parser.parseFromString(`<html><body>${ps.join("\n")}</body></html>`);
            
            for (let y = 0; y < 50; y++) {
            
                console.log("starting y %s", y);
            
                const bytes = htmlDoc.encodeHTML();
                        
                let tx = null;
                console.log("got tx");
                try  {
                    
                    tx = await con.transaction();
                    
                    await tx.execute(`
                        INSERT INTO test(\`test\`, \`uuid\`, \`when\`, \`blob\`) VALUES('hi1', '0000-0000-0000-00000000-000000000000', CURDATE(), :data) ON DUPLICATE KEY UPDATE \`blob\` = :data, \`when\` = CURDATE()
                    `, {data: bytes});
                
                    let returnData = null;
                    await tx.query("SELECT \`blob\` from test where test='hi1' LIMIT 0,1", [], (data) => {
                        returnData = data;
                    });
                    console.log("got data %s c=%s l-%s", typeof returnData, returnData?.constructor?.name, returnData.length);
                    
                    await tx.commit();
                    
                    console.log("comitted");
                } catch(ex) {
                    console.log("MySql fail: %s", ex);
                    console.error(ex);
                    try {
                        await tx?.rollback();
                    } catch(ex) {
                        // care
                    }
                } finally {
                    await tx?.close();
                }
            }
            
            console.log('MySql tx done');

        }

        async function test(){
             
            //await testPg();
            
            await testMySql();
            
        }

        test()
        
        "#,
        );
        let res: JsValueFacade = rt
            .eval(None, script)
            .await
            .map_err(|e| {
                println!("{}", e);
                e
            })
            .map_err(|e| {
                println!("{}", e);
                e
            })
            .ok()
            .expect("script failed");

        println!("{}", res.stringify().as_str());
        if let JsValueFacade::JsPromise { cached_promise } = res {
            let p_res = cached_promise
                .get_promise_result()
                .await
                .ok()
                .expect("get prom res failed");
            match p_res {
                Ok(jsvf) => {
                    println!("prom resolved to {}", jsvf.stringify());
                }
                Err(e) => {
                    panic!("prom rejected: {}", e.stringify());
                }
            }
        } else {
            panic!("did not get a promise");
        }
    }
}
