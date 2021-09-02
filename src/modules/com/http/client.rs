use crate::modules::com::http::request;
use hirofa_utils::js_utils::JsError;
use log::trace;
use quickjs_runtime::quickjs_utils;
use quickjs_runtime::quickjs_utils::primitives;
use quickjs_runtime::reflection::Proxy;
use quickjs_runtime::valueref::JSValueRef;
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Duration;

type HttpClientType = ureq::Agent;

thread_local! {
    static HTTP_CLIENT_INSTANCES: RefCell<HashMap<usize, HttpClientType>> =
         RefCell::new(HashMap::new());

}

fn with_http_client<R, C>(instance_id: &usize, consumer: C) -> R
where
    C: Fn(&mut HttpClientType) -> R,
{
    HTTP_CLIENT_INSTANCES.with(move |instances_rc| {
        let instances = &mut *instances_rc.borrow_mut();
        let i = instances
            .get_mut(instance_id)
            .expect("not a valid instance id");
        consumer(i)
    })
}

pub(crate) fn init_http_client_proxy(
    q_ctx: &QuickJsContext,
    namespace: Vec<&'static str>,
) -> Result<JSValueRef, JsError> {
    Proxy::new()
        .name("Client")
        .namespace(namespace)
        .constructor(|_q_js_rt, instance_id, _args| {
            trace!("Client::constructor");
            HTTP_CLIENT_INSTANCES.with(|instances_rc| {
                let instances = &mut *instances_rc.borrow_mut();

                let agent = ureq::agent();

                instances.insert(instance_id, agent);

                Ok(())
            })
        })
        .finalizer(|_q_ctx, instance_id| {
            trace!("Client::finalizer");
            HTTP_CLIENT_INSTANCES.with(|instances_rc| {
                let instances = &mut *instances_rc.borrow_mut();
                instances.remove(&(instance_id as usize));
            })
        })
        .method("basicAuth", |q_ctx, obj_id, args| {
            trace!("Client::basicAuth {}", obj_id);
            with_http_client(obj_id, |client| {
                // do something with client

                if args.len() != 2 {
                    return Err(JsError::new_string(format!(
                        "basicAuth requires 2 arguments, got {}",
                        args.len()
                    )));
                }

                let user_arg = &args[0];
                let pass_arg = &args[1];

                if !user_arg.is_string() {
                    return Err(JsError::new_str(
                        "basicAuth requires a String as first argument",
                    ));
                }
                if !pass_arg.is_string() {
                    return Err(JsError::new_str(
                        "basicAuth requires a String as second argument",
                    ));
                }

                let user = primitives::to_string_q(q_ctx, &user_arg)?;
                let pass = primitives::to_string_q(q_ctx, &pass_arg)?;

                client.auth(user.as_str(), pass.as_str());
                Ok(quickjs_utils::new_null_ref())
            })
        })
        .method("setHeader", |q_ctx, obj_id, args| {
            trace!("Client::setHeader {}", obj_id);
            with_http_client(obj_id, |client| {
                // do something with client

                if args.len() != 2 {
                    return Err(JsError::new_string(format!(
                        "setHeader requires 2 arguments, got {}",
                        args.len()
                    )));
                }

                let header_arg = &args[0];
                let value_arg = &args[1];

                if !header_arg.is_string() {
                    return Err(JsError::new_str(
                        "setHeader requires a String as first argument",
                    ));
                }
                if !value_arg.is_string() {
                    return Err(JsError::new_str(
                        "setHeader requires a String as second argument",
                    ));
                }

                let header = primitives::to_string_q(q_ctx, &header_arg)?;
                let value = primitives::to_string_q(q_ctx, &value_arg)?;

                client.set(header.as_str(), value.as_str());
                Ok(quickjs_utils::new_null_ref())
            })
        })
        .method("request", |q_ctx, http_client_obj_id, args| {
            if args.len() != 2 {
                return Err(JsError::new_string(
                    "request method requires 2 string arguments: a request method and a path"
                        .to_string(),
                ));
            }

            trace!("Client::request");

            let path_val = &args[1];
            let method_val = &args[0];

            if !path_val.is_string() {
                return Err(JsError::new_str("path argument should be a string"));
            }
            if !method_val.is_string() {
                return Err(JsError::new_str("method argument should be a string"));
            }

            let method = primitives::to_string_q(q_ctx, method_val)?;
            let path = primitives::to_string_q(q_ctx, path_val)?;

            // todo const / webdav methods
            let methods = vec![
                "GET", "POST", "PUT", "DELETE", "HEAD", "OPTIONS", "TRACE", "PATCH",
            ];
            if !methods.contains(&method.as_str()) {
                return Err(JsError::new_string(format!("invalid method: {}", method)));
            }

            with_http_client(http_client_obj_id, |client| {
                let mut request_obj = client.request(method.as_str(), path.as_str());
                request_obj.timeout(Duration::from_secs(10));
                request_obj.timeout_connect(5000);
                request::reg_instance(q_ctx, request_obj)
            })
        })
        .install(q_ctx, false)
}
