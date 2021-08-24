//! runtime agnostic fetch implementation

use crate::features::js_fetch::spec::{do_fetch, FetchInit};
use hirofa_utils::js_utils::adapters::{JsRealmAdapter, JsRuntimeAdapter, JsValueAdapter};
use hirofa_utils::js_utils::facades::{JsRuntimeBuilder, JsRuntimeFacade, JsValueType};
use hirofa_utils::js_utils::JsError;

mod proxies;
pub mod spec;

pub fn init<T: JsRuntimeBuilder>(builder: &mut T) {
    // todo abstract trait for builders
    builder.js_runtime_init_hook(|rt| impl_for_rt(rt));
}

pub fn impl_for_rt<R>(runtime: &R) -> Result<(), JsError>
where
    R: JsRuntimeFacade,
{
    runtime.js_loop_sync(|rta| rta.js_add_realm_init_hook(|_rt, realm| impl_for(realm)))
}

pub fn impl_for<C>(realm: &C) -> Result<(), JsError>
where
    C: JsRealmAdapter + 'static,
{
    realm.js_install_function(
        &[],
        "fetch",
        |_rt, realm, _this_obj, args| {
            //
            // convert vals to fetch options here, make fetch options Send
            //arg0 = url: String
            //arg1 = data: Object
            //if arg0 is not a string the returned promise will reject
            let url: Option<String> =
                if args.len() > 0 && args[0].js_get_type() == JsValueType::String {
                    Some(args[0].js_to_string()?)
                } else {
                    None
                };
            let fetch_init: FetchInit = FetchInit::from_js_object(realm, args.get(1))?;
            let realm_id = realm.js_get_realm_id().to_string();

            let prom = realm.js_promise_create_resolving(
                //
                // do request here and return result as fetch objects
                do_fetch(realm_id, url, fetch_init),
                |realm, res| {
                    // convert result fetch objects to JsValueAdapter here
                    res.to_js_value(realm)
                },
            );

            prom
        },
        2,
    )?;

    proxies::impl_for(realm)
}

#[cfg(test)]
pub mod tests {
    use crate::features::js_fetch::impl_for_rt;
    use crate::tests::init_test_greco_rt;
    use futures::executor::block_on;
    use hirofa_utils::js_utils::facades::values::JsValueFacade;
    use hirofa_utils::js_utils::facades::JsRuntimeFacade;
    use hirofa_utils::js_utils::Script;

    #[test]
    fn test_fetch_generic() {
        let rt = init_test_greco_rt();

        impl_for_rt(&rt).ok().expect("init failed");

        let fetch_fut = rt.js_eval(
            None,
            Script::new("test_fetch_gen.js", "let testFunc = async function() {console.log(1); let fetchRes = await fetch('https://httpbin.org/anything'); let text = await fetchRes.text(); return text;}; testFunc()"),
        );
        let res = block_on(fetch_fut);
        match res {
            Ok(val) => match val {
                JsValueFacade::JsPromise { cached_promise } => {
                    let rti = rt
                        .js_get_runtime_facade_inner()
                        .upgrade()
                        .expect("invalid state");
                    let res_fut = cached_promise.js_get_promise_result(&*rti);
                    let fetch_res = block_on(res_fut);
                    match fetch_res {
                        Ok(v) => match v {
                            Ok(resolved) => {
                                //assert_eq!(resolved.js_get_value_type(), JsValueType::String);
                                println!("resolved to string: {}", resolved.stringify());
                            }
                            Err(rejected) => {
                                panic!("promise was rejected: {}", rejected.stringify());
                            }
                        },
                        Err(e) => {
                            panic!("fetch failed {}", e)
                        }
                    }
                }
                _ => {
                    panic!("result was not a promise")
                }
            },
            Err(e) => {
                panic!("script failed: {}", e);
            }
        }
    }
}
