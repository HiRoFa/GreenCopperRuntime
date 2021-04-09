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
    let namespace = vec!["greco", "com", "http"];
    let http_client_proxy_class = client::init_http_client_proxy(q_ctx, namespace.clone())?;
    let http_request_proxy_class = request::init_http_request_proxy(q_ctx, namespace.clone())?;
    let http_response_proxy_class = response::init_http_response_proxy(q_ctx, namespace.clone())?;

    Ok(vec![
        ("Client", http_client_proxy_class),
        ("Request", http_request_proxy_class),
        ("Response", http_response_proxy_class),
    ])
}

#[cfg(test)]
pub mod tests {

    use crate::tests::init_test_greco_rt;
    use quickjs_runtime::esscript::EsScript;

    #[test]
    fn test_http_client() {
        let rt = init_test_greco_rt();
        let _ = rt
            .eval_sync(EsScript::new(
                "test_http_client1.es",
                "\
            this.test_http_client = async function() {\
                try {\
                let http_mod = await import('greco://http');\
                let test_http_client = new http_mod.Client();\
                test_http_client.setHeader('a', 'b');\
                let req = test_http_client.request('GET', 'https://httpbin.org/anything');\
                req.setHeader('foo', 'bar');\
                let response = await req.send();\
                let txt = response.text;\
                return('response text = ' + txt);\
                } catch(ex){\
                    console.error('test_http_client failed at %s', '' + ex);\
                    throw Error('test_http_client failed at ' + ex);
                }\
            }\
            ",
            ))
            .ok()
            .expect("script failed");

        let esvf_prom = rt
            .call_function_sync(vec![], "test_http_client", vec![])
            .ok()
            .expect("func invoc failed");
        let esvf_res = esvf_prom.get_promise_result_sync();
        match esvf_res {
            Ok(o) => {
                assert!(o.is_string());
                let res_str = o.get_str();
                log::debug!("response text = {}", res_str);
                assert!(res_str.starts_with("response text = "));
            }
            Err(e) => {
                panic!("failed {}", e.get_str());
            }
        }
    }

    #[test]
    fn test_http_client_post() {
        let rt = init_test_greco_rt();
        let _ = rt
            .eval_sync(EsScript::new(
                "test_http_client1.es",
                "\
            this.test_http_client = async function() {\
                let http_mod = await import('greco://http');\
                let test_http_client = new http_mod.Client();\
                test_http_client.setHeader('a', 'b');\
                let req = test_http_client.request('POST', 'https://httpbin.org/post');\
                req.setHeader('foo', 'bar');\
                let response = await req.send('hello posty world');\
                let txt = response.text;\
                return('response text = ' + txt);\
            }\
            ",
            ))
            .ok()
            .expect("scriopt failed");

        let esvf_prom = rt
            .call_function_sync(vec![], "test_http_client", vec![])
            .ok()
            .expect("func invoc faled");
        let esvf_res = esvf_prom.get_promise_result_sync();
        match esvf_res {
            Ok(o) => {
                assert!(o.is_string());
                let res_str = o.get_str();
                log::debug!("response text = {}", res_str);
                assert!(res_str.starts_with("response text = "));
                assert!(res_str.contains("\"data\": \"hello posty world\""));
                assert!(res_str.contains("\"A\": \"b\""));
            }
            Err(e) => {
                panic!("failed {}", e.get_str());
            }
        }
    }
}
