use quickjs_runtime::eserror::EsError;
use quickjs_runtime::quickjs_utils::{json, primitives};
use quickjs_runtime::quickjscontext::QuickJsContext;
use quickjs_runtime::reflection;
use quickjs_runtime::reflection::Proxy;
use quickjs_runtime::valueref::JSValueRef;
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::replace;

pub(crate) struct UreqResponseWrapper {
    pub(crate) delegate: Option<ureq::Response>,
}

type HttpResponseType = UreqResponseWrapper;

thread_local! {
    static HTTP_RESPONSE_INSTANCES: RefCell<HashMap<usize, HttpResponseType>> =
         RefCell::new(HashMap::new());
}

fn with_http_response<R, C>(instance_id: &usize, consumer: C) -> R
where
    C: Fn(&mut HttpResponseType) -> R,
{
    HTTP_RESPONSE_INSTANCES.with(move |instances_rc| {
        let instances = &mut *instances_rc.borrow_mut();
        let i = instances
            .get_mut(instance_id)
            .expect("not a valid instance id");
        consumer(i)
    })
}

pub(crate) fn reg_instance(
    q_ctx: &QuickJsContext,
    response_obj: HttpResponseType,
) -> Result<JSValueRef, EsError> {
    let resp_proxy = reflection::get_proxy(q_ctx, "greco.com.http.Response")
        .expect("could not find greco.com.http.Response proxy");

    let instance_res = reflection::new_instance2(&resp_proxy, q_ctx)?;

    let response_obj_id = instance_res.0;
    let response_instance_ref = instance_res.1;

    HTTP_RESPONSE_INSTANCES.with(|requests_rc| {
        let requests = &mut *requests_rc.borrow_mut();
        requests.insert(response_obj_id, response_obj);
    });

    Ok(response_instance_ref)
}

pub(crate) fn init_http_response_proxy(
    q_ctx: &QuickJsContext,
    namespace: Vec<&'static str>,
) -> Result<JSValueRef, EsError> {
    Proxy::new()
        .name("Response")
        .namespace(namespace)
        .method("json", |q_ctx, obj_id, _args| {
            // todo, should return a promise, at which point things will suck because the response in in an thread local in the wrong thread.,....
            // so box in Arc in the hashmap and move a clone of that arc to the producer thread?
            let text_content_opt = with_http_response(obj_id, |response_wrapper| {
                response_wrapper.delegate.as_ref()?; // this returns None if delegate is none
                let ureq_response: ureq::Response =
                    replace(&mut response_wrapper.delegate, None).unwrap();

                let res = ureq_response.into_string();
                let text = res.expect("request failed");
                log::trace!("got text from http resp: {}", text);
                Some(text)
            });

            if let Some(text_content) = text_content_opt {
                json::parse_q(q_ctx, text_content.as_str())
            } else {
                Err(EsError::new_str("response was already consumed"))
            }
        })
        .getter_setter(
            "text",
            |q_ctx, obj_id| {
                let text_content_opt = with_http_response(obj_id, |response_wrapper| {
                    response_wrapper.delegate.as_ref()?;
                    let ureq_response: ureq::Response =
                        replace(&mut response_wrapper.delegate, None).unwrap();

                    let res = ureq_response.into_string();
                    let text = res.expect("request failed");
                    log::trace!("got text from http resp: {}", text);
                    Some(text)
                });

                if let Some(text_content) = text_content_opt {
                    primitives::from_string_q(q_ctx, text_content.as_str())
                } else {
                    Err(EsError::new_str("response was already consumed"))
                }
            },
            |_q_js_rt, _obj_id, _v| Ok(()),
        )
        .install(q_ctx, false)
}
