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
use hirofa_utils::js_utils::JsError;
use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;
use quickjs_runtime::esvalue::EsValueFacade;
use quickjs_runtime::quickjs_utils;
use quickjs_runtime::quickjs_utils::primitives;
use quickjs_runtime::quickjscontext::QuickJsContext;
use quickjs_runtime::quickjsruntime::{NativeModuleLoader, QuickJsRuntime};
use quickjs_runtime::reflection::Proxy;
use quickjs_runtime::valueref::JSValueRef;
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
    // todo
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

impl NativeModuleLoader for MysqlModuleLoader {
    fn has_module(&self, _q_ctx: &QuickJsContext, module_name: &str) -> bool {
        module_name.eq("greco://mysql")
    }

    fn get_module_export_names(&self, _q_ctx: &QuickJsContext, _module_name: &str) -> Vec<&str> {
        vec!["Connection"]
    }

    fn get_module_exports(
        &self,
        q_ctx: &QuickJsContext,
        _module_name: &str,
    ) -> Vec<(&str, JSValueRef)> {
        init_exports(q_ctx).ok().expect("init mysql exports failed")
    }
}

pub(crate) fn init(builder: EsRuntimeBuilder) -> EsRuntimeBuilder {
    builder.native_module_loader(Box::new(MysqlModuleLoader {}))
}

fn init_exports(q_ctx: &QuickJsContext) -> Result<Vec<(&'static str, JSValueRef)>, JsError> {
    let myql_connection_proxy_class = Proxy::new()
        .namespace(vec!["greco", "db", "mysql"])
        .name("Connection")
        .constructor(|q_ctx, id, args| {
            let con = MysqlConnection::new(q_ctx, &args)?;
            store_connection(id, con);
            Ok(())
        })
        .method("transaction", |_q_ctx, _id, _args| {
            Ok(quickjs_utils::new_null_ref())
        })
        .method("query", |q_ctx, id, args| {
            // todo think up a macro for this?
            // 3 args, second may be null
            if args.len() != 3 {
                Err(JsError::new_str("query requires 3 arguments (query: String, params: Array, rowConsumer: Function)"))
            } else {
                // todo

                let query = primitives::to_string_q(q_ctx, &args[0])?;
                let params = EsValueFacade::from_jsval(q_ctx, &args[1])?;
                let row_consumer = EsValueFacade::from_jsval(q_ctx, &args[2])?;

                with_connection( id, |con| {
                    let mut esvf = con.query(query.as_str(), params, row_consumer)?;
                    esvf.as_js_value(q_ctx)
                })
            }

        })
        .finalizer(|_q_ctx, id| {
            QuickJsRuntime::do_with(|_q_js_rt| {
                drop_connection(&id);
            })
        })
        .install(q_ctx, false)?;

    Ok(vec![("Connection", myql_connection_proxy_class)])
}
