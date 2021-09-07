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
//!     await con.execute("insert into table1(name, lastName) values(?, ?)", ['Pete', 'Peterson'], ['Andrew', 'Anderson']);
//!
//!     let q1_objects = await con.query("select * from something where id > ? and age < ?", [1, 123], (row) => {
//!        let name = row[0];
//!         let age = row[1];
//!         console.log("a name = %s, age = %s", name, age);
//!         return {name, age};
//!     });
//!     // q1_objects is an array like [{name: 'n1', age: 21}, {name: 'n2', age: 37}, etc..]
//!
//!     let transaction = await con.transaction();
//!     try {
//!         // all methods return a promise so execution never blocks
//!         await transaction.execute("insert into table1(name) values('?')", ['Harry'], ['Henry']);
//!         let ct_rows = await transaction.query("select count(*) from table1", [], (row) => {
//!             return {count: row[0]};
//!         });
//!         await transaction.commit();
//!     } finally {
//!         // returns a promise await for eror handling.
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
    let myql_connection_proxy_class = JsProxy::new(&["greco", "db", "mysql"], "Connection")
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
                Err(JsError::new_str("query requires 3 arguments (query: String, params: Array, rowConsumer: Function)"))
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
        });
    let res = realm.js_proxy_install(myql_connection_proxy_class, false)?;

    Ok(vec![("Connection", res)])
}
