use crate::modules::com::http::response;
use crate::modules::com::http::response::UreqResponseWrapper;
use log::trace;
use quickjs_runtime::eserror::EsError;
use quickjs_runtime::esruntime_utils::promises;
use quickjs_runtime::quickjs_utils::{functions, json, primitives};
use quickjs_runtime::quickjscontext::QuickJsContext;
use quickjs_runtime::quickjsruntime::QuickJsRuntime;
use quickjs_runtime::reflection::Proxy;
use quickjs_runtime::valueref::JSValueRef;
use quickjs_runtime::{quickjs_utils, reflection};
use std::cell::RefCell;
use std::collections::HashMap;

type HttpRequestType = ureq::Request;

thread_local! {
    static HTTP_REQUEST_INSTANCES: RefCell<HashMap<usize, HttpRequestType>> =
         RefCell::new(HashMap::new());
}

pub(crate) fn with_http_request<R, C>(instance_id: &usize, consumer: C) -> R
where
    C: Fn(&mut HttpRequestType) -> R,
{
    HTTP_REQUEST_INSTANCES.with(move |instances_rc| {
        let instances = &mut *instances_rc.borrow_mut();
        let i = instances
            .get_mut(instance_id)
            .expect("not a valid instance id");
        consumer(i)
    })
}

pub(crate) fn reg_instance(
    q_ctx: &QuickJsContext,
    request_obj: HttpRequestType,
) -> Result<JSValueRef, EsError> {
    let req_proxy = reflection::get_proxy(q_ctx, "greco.com.http.Request")
        .expect("could not find greco.com.http.Request proxy");

    let instance_res = reflection::new_instance2(&req_proxy, q_ctx)?;

    let request_obj_id = instance_res.0;
    let request_instance_ref = instance_res.1;

    HTTP_REQUEST_INSTANCES.with(|requests_rc| {
        let requests = &mut *requests_rc.borrow_mut();
        requests.insert(request_obj_id, request_obj);
    });

    Ok(request_instance_ref)
}

pub(crate) fn init_http_request_proxy(
    q_ctx: &QuickJsContext,
    namespace: Vec<&'static str>,
) -> Result<JSValueRef, EsError> {
    Proxy::new()
        .name("Request")
        .namespace(namespace)
        .method("setHeader", |q_ctx, obj_id, args| {
            if args.len() != 2 {
                return Err(EsError::new_str("setHeader requires two string arguments"));
            }
            let name_arg = &args[0];
            let value_arg = &args[1];

            if !value_arg.is_string() || !name_arg.is_string() {
                return Err(EsError::new_str("setHeader requires two string arguments"));
            }

            let name_str = primitives::to_string_q(q_ctx, name_arg)?;
            let value_str = primitives::to_string_q(q_ctx, value_arg)?;

            with_http_request(obj_id, |req| {
                // todo args and stuff
                log::debug!("setting header in req {} to {}", name_str, value_str);
                req.set(name_str.as_str(), value_str.as_str());
            });
            Ok(quickjs_utils::new_null_ref())
        })
        .method("send", |q_ctx, obj_id, args| {
            trace!("Request::send");

            // first arg can be object or string or byte[](UInt8Array)
            let content_opt: Option<String> = if args.len() > 0 {
                let arg = &args[0];
                let s = if arg.is_object() {
                    let val_ref = json::stringify_q(q_ctx, arg, None).ok().unwrap();
                    primitives::to_string_q(q_ctx, &val_ref).ok().unwrap()
                } else {
                    functions::call_to_string_q(q_ctx, arg).ok().unwrap()
                };

                Some(s)
            } else {
                None
            };

            let mut req_clone = with_http_request(obj_id, |req| req.build());

            let es_rt_ref = QuickJsRuntime::do_with(|q_js_rt| q_js_rt.get_rt_ref());

            if let Some(es_rt) = es_rt_ref {
                promises::new_resolving_promise(
                    q_ctx,
                    move || {
                        // producer, make request here and return result

                        let response = if let Some(content) = content_opt {
                            req_clone.send_string(content.as_str())
                        } else {
                            req_clone.call()
                        };

                        if response.ok() {
                            Ok(response)
                        } else {
                            Err(response
                                .into_string()
                                .ok()
                                .expect("could not get response error as string"))
                        }
                    },
                    |q_ctx, response_obj| {
                        // put res in autoidmap and return new proxy instance here

                        let response_ref = response::reg_instance(
                            q_ctx,
                            UreqResponseWrapper {
                                delegate: Some(response_obj),
                            },
                        );

                        response_ref
                    },
                    es_rt,
                )
            } else {
                Ok(quickjs_utils::new_null_ref())
            }
        })
        .install(q_ctx, false)
}
