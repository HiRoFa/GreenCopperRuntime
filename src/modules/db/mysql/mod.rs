//!
//! # The MYSQL module
//!
//! work in progress
//!
//! ## Example
//!
//! ```javascript
//!!
//! async function test_mysql(){
//!     let mysql_mod = await import('greco://mysql');
//!   
//!     let url = "localhost";
//!     let port = 3306;
//!     let user = null;
//!     let pass = null;
//!     let dbSchema = "esses";
//!     let con = new mysql_mod.Connection(url, port, user, pass, dbSchema);
//!     // params and batches
//!     await con.execute("insert into table1(name, lastName) values(:name, :lastName)", {name: 'Pete', lastName: 'Peterson'}, {name: 'Andrew', lastName: 'Anderson'});
//!
//!     let q1_objects = await con.query("select * from something where id > :id and age < :age", {id: 1, age: 123}, (name, age) => {
//!         console.log("a name = %s, age = %s", name, age);
//!         return {name, age};
//!     });
//!     // q1_objects is an array like [{name: 'n1', age: 21}, {name: 'n2', age: 37}, etc..]
//!
//!     let transaction = await con.transaction();
//!     try {
//!         // all methods return a promise so execution never blocks
//!         await transaction.execute("insert into table1(name) values(:name)", {name: 'Harry'}, {name: 'Henry'});
//!         let ct_rows = await transaction.query("select count(*) from table1", [], (ct) => {
//!             return {count: ct};
//!         });
//!         await transaction.commit();
//!     } finally {
//!         // returns a promise await for error handling.
//!         await transaction.close();
//!     }
//! }
//!
//! test_mysql().then(() => {
//!     console.log("done");
//! }).catch((ex) => {
//!     console.log("test failed: %s", ex);
//! });
//!
//! ```
//!

use crate::modules::db::mysql::connection::MysqlConnection;
use crate::modules::db::mysql::transaction::MysqlTransaction;
use hirofa_utils::auto_id_map::AutoIdMap;
use quickjs_runtime::builder::QuickJsRuntimeBuilder;
use quickjs_runtime::jsutils::jsproxies::JsProxy;
use quickjs_runtime::jsutils::modules::NativeModuleLoader;
use quickjs_runtime::jsutils::JsError;
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use quickjs_runtime::quickjsvalueadapter::QuickJsValueAdapter;
use std::cell::RefCell;
use std::collections::HashMap;

pub mod connection;
pub mod transaction;

thread_local! {
    static CONNECTIONS: RefCell<HashMap< usize, MysqlConnection>> = RefCell::new(HashMap::new());
    static TRANSACTIONS: RefCell<AutoIdMap<MysqlTransaction>> = RefCell::new(AutoIdMap::new());
}

fn with_connection<R, C: FnOnce(&MysqlConnection) -> R>(
    proxy_instance_id: &usize,
    consumer: C,
) -> R {
    CONNECTIONS.with(|rc| {
        let map = &*rc.borrow();
        let con: &MysqlConnection = map.get(proxy_instance_id).expect("no such Connection");
        consumer(con)
    })
}

fn with_transaction<R, C: FnOnce(&MysqlTransaction) -> R>(
    proxy_instance_id: &usize,
    consumer: C,
) -> R {
    TRANSACTIONS.with(|rc| {
        let map = &*rc.borrow();
        let con: &MysqlTransaction = map.get(proxy_instance_id).expect("no such Transaction");
        consumer(con)
    })
}

fn with_transaction_mut<R, C: FnOnce(&mut MysqlTransaction) -> R>(
    proxy_instance_id: &usize,
    consumer: C,
) -> R {
    TRANSACTIONS.with(|rc| {
        let map = &mut *rc.borrow_mut();
        let con: &mut MysqlTransaction =
            map.get_mut(proxy_instance_id).expect("no such Transaction");
        consumer(con)
    })
}

fn store_connection(proxy_instance_id: usize, con: MysqlConnection) {
    CONNECTIONS.with(|rc| {
        let map = &mut *rc.borrow_mut();
        map.insert(proxy_instance_id, con);
    })
}

fn drop_connection(proxy_instance_id: &usize) {
    CONNECTIONS.with(|rc| {
        let map = &mut *rc.borrow_mut();
        map.remove(proxy_instance_id);
    })
}

fn store_transaction(tx: MysqlTransaction) -> usize {
    TRANSACTIONS.with(|rc| {
        let map = &mut *rc.borrow_mut();
        map.insert(tx)
    })
}

fn drop_transaction(proxy_instance_id: &usize) {
    TRANSACTIONS.with(|rc| {
        let map = &mut *rc.borrow_mut();
        map.remove(proxy_instance_id);
    })
}

struct MysqlModuleLoader {}

impl NativeModuleLoader for MysqlModuleLoader {
    fn has_module(&self, _realm: &QuickJsRealmAdapter, module_name: &str) -> bool {
        module_name.eq("greco://mysql")
    }

    fn get_module_export_names(
        &self,
        _realm: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<&str> {
        vec!["Connection"]
    }

    fn get_module_exports(
        &self,
        realm: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<(&str, QuickJsValueAdapter)> {
        init_exports(realm).expect("init mysql exports failed")
    }
}

pub(crate) fn init(builder: QuickJsRuntimeBuilder) -> QuickJsRuntimeBuilder {
    builder.js_native_module_loader(MysqlModuleLoader {})
}

fn init_exports(
    realm: &QuickJsRealmAdapter,
) -> Result<Vec<(&'static str, QuickJsValueAdapter)>, JsError> {
    let mysql_connection_proxy_class = create_mysql_connection_proxy(realm);
    let mysql_transaction_proxy_class = create_mysql_transaction_proxy(realm);
    let con_res = realm.install_proxy(mysql_connection_proxy_class, false)?;
    let tx_res = realm.install_proxy(mysql_transaction_proxy_class, false)?;

    Ok(vec![("Connection", con_res), ("Transaction", tx_res)])
}

pub(crate) fn create_mysql_transaction_proxy(_realm: &QuickJsRealmAdapter) -> JsProxy {
    JsProxy::new().namespace(&["greco", "db", "mysql"]).name("Transaction")
        .event_target()
        .method("commit", |runtime, realm, id, _args| {
            with_transaction_mut( id, |tx| {
                tx.commit(runtime, realm, *id)
            })
        })
        .method("rollback", |runtime, realm, id, _args| {
            with_transaction_mut( id, |tx| {
                tx.rollback(runtime, realm)
            })
        })
        .method("close", |runtime, realm, id, _args| {
            with_transaction( id, |tx| {
                tx.close_tx(runtime, realm)
            })
        })
        .method("query", |runtime, realm, id, args| {

            // todo think up a macro for this?
            // 3 args, second may be null
            if args.len() != 3 {
                Err(JsError::new_str("query requires 3 arguments (query: String, params: Object, rowConsumer: Function)"))
            } else {
                // todo

                let query = args[0].to_string()?;

                let params = &args[1];
                let row_consumer = &args[2];

                with_transaction( id, |tx| {
                    tx.query(runtime, realm, query.as_str(), params, row_consumer)
                })
            }

        })
        .method("execute", |runtime, realm, id, args| {
            // todo think up a macro for this?
            // 3 args, second may be null
            if args.len() < 2 {
                Err(JsError::new_str("execute requires at least 2 arguments (query: String, ...params: Array<Object>)"))
            } else {
                // todo

                let query = args[0].to_string()?;

                let params: Vec<&QuickJsValueAdapter> = args[1..args.len()].iter().collect();

                with_transaction( id, |tx| {
                    tx.execute(runtime, realm, query.as_str(), &params)
                })
            }

        })
        .finalizer(|_rt, _realm, id| {
            drop_transaction(&id);
        })
}

pub(crate) fn create_mysql_connection_proxy(_realm: &QuickJsRealmAdapter) -> JsProxy {
    JsProxy::new().namespace(&["greco", "db", "mysql"]).name("Connection")
        .constructor(|runtime, realm, instance_id, args| {
            let con = MysqlConnection::new(runtime, realm, args)?;
            store_connection(instance_id, con);
            Ok(())
        })
        .method("transaction", |_runtime, realm, id, _args| {
            // todo options like isolation/readonly
            with_connection( id, |con| {
                 con.start_transaction(realm)
            })
        })
        .method("query", |runtime, realm, id, args| {
            // todo think up a macro for this?
            // 3 args, second may be null
            if args.len() != 3 {
                Err(JsError::new_str("query requires 3 arguments (query: String, params: Object, rowConsumer: Function)"))
            } else {
                // todo

                let query = args[0].to_string()?;

                let params = &args[1];
                let row_consumer = &args[2];

                with_connection( id, |con| {
                    con.query(runtime, realm, query.as_str(), params, row_consumer)
                })
            }

        })
        .method("execute", |runtime, realm, id, args| {
            // todo think up a macro for this?
            // 3 args, second may be null
            if args.len() < 2 {
                Err(JsError::new_str("execute requires at least 2 arguments (query: String, ...params: Array<Object>)"))
            } else {
                // todo

                let query = args[0].to_string()?;

                let params: Vec<&QuickJsValueAdapter> = args[1..args.len()].iter().collect();

                with_connection( id, |con| {
                    con.execute(runtime, realm, query.as_str(), &params)
                })
            }

        })
        .finalizer(|_rt, _realm, id| {
            drop_connection(&id);
        })
}

#[cfg(test)]
pub mod tests {
    use futures::executor::block_on;
    //use log::LevelFilter;
    use quickjs_runtime::builder::QuickJsRuntimeBuilder;
    use quickjs_runtime::jsutils::Script;
    use quickjs_runtime::values::JsValueFacade;

    //#[test]
    fn _test_params() {
        //simple_logging::log_to_stderr(LevelFilter::Info);

        let builder = QuickJsRuntimeBuilder::new();
        let builder = crate::init_greco_rt(builder);
        let rt = builder.build();

        //simple_logging::log_to_stderr(log::LevelFilter::Trace);

        let script = Script::new(
            "test_mysql.js",
            r#"
        
        async function test() {
            let mysqlMod = await import('greco://mysql');
            let host = '192.168.10.43';
            let port = 3306;
            let user = 'hirofa';
            let pass = 'hirofa';
            let db = 'hirofa_testdb';
            let con = new mysqlMod.Connection(host, port, user, pass, db);
            
            await con.query('select * from test where \'test\' = ?', ['test'], (...rows) => {
                for (let x = 0; x < rows.length; x++) {
                    console.log('row %s = %s', x, rows[x]);
                }

            });
            
            await con.query('select * from test where \'test\' = :a', {a: 'test'}, (...rows) => {
                for (let x = 0; x < rows.length; x++) {
                    console.log('named row %s = %s', x, rows[x]);
                }
            });
            
            let ct = await con.execute('delete from test', []);
            console.log('deleted rows %s', ct);
            ct = await con.execute('delete from test', []);
            console.log('deleted rows %s', ct);
            
            await con.execute('insert into test(id) values(?)', [4], [8], [12]);
            
            let tables = await con.query('show tables', null, (row1) => {
                return row1;
            });
            
            for (let table of tables) {
                console.log('found table %s', table);            
            }
            
        }
        
        test()
        
        "#,
        );
        let res: JsValueFacade = block_on(rt.eval(None, script)).ok().expect("script failed");

        println!("{}", res.stringify());
        if let JsValueFacade::JsPromise { cached_promise } = res {
            let p_res = block_on(cached_promise.get_promise_result())
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
