//! runtime agnostic fetch implementation

use crate::features::js_fetch::spec::{do_fetch, FetchInit};
use quickjs_runtime::builder::QuickJsRuntimeBuilder;
use quickjs_runtime::facades::QuickJsRuntimeFacade;
use quickjs_runtime::jsutils::{JsError, JsValueType};
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;

mod proxies;
pub mod spec;

pub fn init(builder: QuickJsRuntimeBuilder) -> QuickJsRuntimeBuilder {
    // todo abstract trait for builders
    builder.js_runtime_init_hook(impl_for_rt)
}

pub fn impl_for_rt(runtime: &QuickJsRuntimeFacade) -> Result<(), JsError> {
    runtime.loop_sync_mut(|rta| rta.add_realm_init_hook(|_rt, realm| impl_for(realm)))
}

pub fn impl_for(realm: &QuickJsRealmAdapter) -> Result<(), JsError> {
    realm.install_function(
        &[],
        "fetch",
        |_rt, realm, _this_obj, args| {
            //
            // convert vals to fetch options here, make fetch options Send
            //arg0 = url: String
            //arg1 = data: Object
            //if arg0 is not a string the returned promise will reject
            let url: Option<String> =
                if !args.is_empty() && args[0].get_js_type() == JsValueType::String {
                    Some(args[0].to_string()?)
                } else {
                    None
                };
            let fetch_init: FetchInit = FetchInit::from_js_object(realm, args.get(1))?;
            let realm_id = realm.get_realm_id().to_string();

            realm.create_resolving_promise_async(
                //
                // do request here and return result as fetch objects
                do_fetch(realm_id, url, fetch_init),
                |realm, res| {
                    // convert result fetch objects to JsValueAdapter here
                    res.to_js_value(realm)
                },
            )
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
    use quickjs_runtime::jsutils::Script;
    use quickjs_runtime::values::JsValueFacade;

    #[test]
    fn test_fetch_generic() {
        let rt = init_test_greco_rt();

        impl_for_rt(&rt).ok().expect("init failed");

        let fetch_fut = rt.eval(
            None,
            Script::new("test_fetch_gen.js", "let testFunc = async function() {console.log(1); let fetchRes = await fetch('https://httpbin.org/anything', {headers: {myHeader: ['a', 'b']}}); let text = await fetchRes.text(); return text;}; testFunc()"),
        );
        let res = block_on(fetch_fut);
        match res {
            Ok(val) => match val {
                JsValueFacade::JsPromise { cached_promise } => {
                    let res_fut = cached_promise.get_promise_result();
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
