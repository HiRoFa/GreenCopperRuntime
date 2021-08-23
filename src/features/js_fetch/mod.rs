//! runtime agnostic fetch implementation

use crate::features::js_fetch::spec::{do_fetch, FetchInit};
use hirofa_utils::js_utils::adapters::{JsRealmAdapter, JsRuntimeAdapter, JsValueAdapter};
use hirofa_utils::js_utils::facades::{JsRuntimeBuilder, JsRuntimeFacade, JsValueType};
use hirofa_utils::js_utils::JsError;

pub mod spec;

pub fn init<T: JsRuntimeBuilder>(builder: &mut T) {
    // todo abstract trait for builders
    builder.js_runtime_init_hook(|rt| {
        rt.js_loop_sync(|rta| rta.js_add_realm_init_hook(|_rt, realm| impl_for(realm)))
    });
}

pub fn impl_for<C>(ctx: &C) -> Result<(), JsError>
where
    C: JsRealmAdapter + 'static,
{
    ctx.js_install_function(
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
            let fetch_init: FetchInit = FetchInit::from_js_object(realm, args.get(1));
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
    )
}

#[cfg(test)]
pub mod tests {
    use crate::features::js_fetch::init;
    use hirofa_utils::js_utils::Script;
    use quickjs_runtime::builder::QuickJsRuntimeBuilder;

    #[test]
    fn test_fetch_generic() {
        let mut rtb = QuickJsRuntimeBuilder::new();
        init(&mut rtb);
        let rt = rtb.build();

        let res = rt.eval_sync(Script::new(
            "test_fetch_gen.es",
            "fetch('https://httpbin.org/anything')",
        ));
        match res {
            Ok(val) => {
                assert!(val.is_promise());
            }
            Err(e) => {
                panic!("script failed: {}", e);
            }
        }
    }
}
