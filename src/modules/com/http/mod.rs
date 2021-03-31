//! # Http module
//!
//! The http module provides a more manageable httpclient than the fetch api can provide
//!
//! # exports
//!
//! ## Client
//!
//! * retains cookies
//! * default http headers
//!
//! ### setHeader
//!
//! ### basicAuth
//!
//! ## Request
//!
//! * setHeader(name, val)
//! * async send()
//!
//! ## Response
//!
//! * get text()
//!
//! ## do requests
//!
//! ```javascript
//! async function testHttp() {
//!     let http_mod = await import("greco://http");
//!     let client = new http_mod.Client();
//!     client.basicAuth("userA", "passB");
//!     client.setHeader("X-Api-Key", "12345");
//!
//!     let req = client.request("GET", "https://foo.com");
//!     req.setHeader("headerA", "a");
//!     req.setHeader("headerB", "b");
//!
//!     let response = await req.send();
//!     let txt = response.text;
//!     console.log("got response: %s", txt);
//! }
//! ```
//!

use quickjs_runtime::eserror::EsError;
use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;
use quickjs_runtime::quickjscontext::QuickJsContext;
use quickjs_runtime::quickjsruntime::NativeModuleLoader;
use quickjs_runtime::reflection::Proxy;
use quickjs_runtime::valueref::JSValueRef;

mod client;
mod request;
mod response;

struct HttpModuleLoader {}

impl NativeModuleLoader for HttpModuleLoader {
    fn has_module(&self, _q_ctx: &QuickJsContext, module_name: &str) -> bool {
        module_name.eq("greco://http")
    }

    fn get_module_export_names(&self, _q_ctx: &QuickJsContext, _module_name: &str) -> Vec<&str> {
        vec!["Client"]
    }

    fn get_module_exports(
        &self,
        q_ctx: &QuickJsContext,
        _module_name: &str,
    ) -> Vec<(&str, JSValueRef)> {
        init_exports(q_ctx).ok().expect("init http exports failed")
    }
}

pub(crate) fn init(builder: EsRuntimeBuilder) -> EsRuntimeBuilder {
    builder.native_module_loader(Box::new(HttpModuleLoader {}))
}

fn init_exports(q_ctx: &QuickJsContext) -> Result<Vec<(&'static str, JSValueRef)>, EsError> {
    let http_client_proxy_class = Proxy::new()
        .namespace(vec!["greco", "io", "http"])
        .name("Client")
        .install(q_ctx, false)?;
    Ok(vec![("Client", http_client_proxy_class)])
}
