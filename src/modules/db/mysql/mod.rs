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
//! # todo
//!
//! the row object, should that just be an array ```['Harry', 39]``` or an object ```{name: "Harry", age: 39}```
//!
//! or both by different forEach methods? or arg?
//!
//! ```rs.forEach()``` and ```rs.forEachNamed()```?
//!
//! or return a proxy obj with dynamic getters for names and indices?
//!
//!

use crate::modules::db::mysql::connection::MysqlConnection;
use hirofa_utils::js_utils::adapters::proxies::JsProxy;
use hirofa_utils::js_utils::adapters::{JsRealmAdapter, JsValueAdapter};
use hirofa_utils::js_utils::facades::JsRuntimeBuilder;
use hirofa_utils::js_utils::modules::NativeModuleLoader;
use hirofa_utils::js_utils::JsError;
use std::cell::RefCell;
use std::collections::HashMap;

pub mod connection;
pub mod transaction;

thread_local! {
    static CONNECTIONS: RefCell<HashMap< usize, MysqlConnection>> = RefCell::new(HashMap::new());
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

struct MysqlModuleLoader {}

impl<R: JsRealmAdapter + 'static> NativeModuleLoader<R> for MysqlModuleLoader {
    fn has_module(&self, _realm: &R, module_name: &str) -> bool {
        module_name.eq("greco://mysql")
    }

    fn get_module_export_names(&self, _realm: &R, _module_name: &str) -> Vec<&str> {
        vec!["Connection"]
    }

    fn get_module_exports(
        &self,
        realm: &R,
        _module_name: &str,
    ) -> Vec<(&str, R::JsValueAdapterType)> {
        init_exports(realm).ok().expect("init mysql exports failed")
    }
}

pub(crate) fn init<B: JsRuntimeBuilder>(builder: B) -> B {
    builder.js_native_module_loader(MysqlModuleLoader {})
}

fn init_exports<R: JsRealmAdapter + 'static>(
    realm: &R,
) -> Result<Vec<(&'static str, R::JsValueAdapterType)>, JsError> {
    let mysql_connection_proxy_class = create_mysql_connection_proxy(realm);
    let res = realm.js_proxy_install(mysql_connection_proxy_class, false)?;

    Ok(vec![("Connection", res)])
}

pub(crate) fn create_mysql_connection_proxy<R: JsRealmAdapter + 'static>(_realm: &R) -> JsProxy<R> {
    JsProxy::new(&["greco", "db", "mysql"], "Connection")
        .set_constructor(|runtime, realm: &R, instance_id, args| {
            let con = MysqlConnection::new(runtime, realm, args)?;
            store_connection(instance_id, con);
            Ok(())
        })
        .add_method("transaction", |_runtime, realm, _id, _args| {
            realm.js_null_create()
        })
        .add_method("query", |runtime, realm: &R, id, args| {
            // todo think up a macro for this?
            // 3 args, second may be null
            if args.len() != 3 {
                Err(JsError::new_str("query requires 3 arguments (query: String, params: Object, rowConsumer: Function)"))
            } else {
                // todo

                let query = args[0].js_to_string()?;

                let params = &args[1];
                let row_consumer = &args[2];

                with_connection( &id, |con| {
                    con.query(runtime, realm, query.as_str(), params, row_consumer)
                })
            }

        })
        .set_finalizer(|_rt, _realm, id| {
            drop_connection(&id);
        })
}

#[cfg(test)]
pub mod tests {
    use futures::executor::block_on;
    use hirofa_utils::js_utils::facades::values::JsValueFacade;
    use hirofa_utils::js_utils::facades::JsRuntimeFacade;
    use hirofa_utils::js_utils::Script;
    use log::LevelFilter;
    use quickjs_runtime::builder::QuickJsRuntimeBuilder;

    #[test]
    fn test_params() {
        let builder = QuickJsRuntimeBuilder::new();
        let builder = crate::init_greco_rt(builder);
        let rt = builder.build();

        simple_logging::log_to_stderr(LevelFilter::Trace);

        let script = Script::new(
            "test_mysql.js",
            r#"
        
        async function test() {
            let mysqlMod = await import('greco://mysql');
            let host = '127.0.0.1';
            let port = 3307;
            let user = 'test';
            let pass = 'test';
            let db = 'testdb';
            let con = new mysqlMod.Connection(host, port, user, pass, db);
            
            await con.query('select * from test where \'test\' = ?', ['test'], (...rows) => {
                console.log('row %s', rows[0]);
            });
            
            await con.query('select * from test where \'test\' = :a', {a: 'test'}, (...rows) => {
                console.log('named row %s', rows[0]);
            });
            
        }
        
        test()
        
        "#,
        );
        let res: JsValueFacade = block_on(rt.js_eval(None, script))
            .ok()
            .expect("script failed");

        println!("{}", res.stringify());
        if let JsValueFacade::JsPromise { cached_promise } = res {
            let rti_weak = rt.js_get_runtime_facade_inner();
            let rti = rti_weak.upgrade().expect("invalid state");
            let p_res = block_on(cached_promise.js_get_promise_result(&*rti))
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
