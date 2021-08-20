//! runtime agnostic fetch implementation

use hirofa_utils::js_utils::adapters::{JsPromiseAdapter, JsRealmAdapter, JsRuntimeAdapter};
use hirofa_utils::js_utils::facades::{JsRuntimeBuilder, JsRuntimeFacade};
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
    C: JsRealmAdapter,
{
    ctx.js_install_function(
        &[],
        "fetch",
        |rt, realm, _this_obj, _args| {
            //
            let prom = realm.js_promise_create()?;

            //prom.js_promise_resolve()
            //prom.js_promise_reject()

            Ok(prom.js_promise_get_value())
        },
        2,
    )
}

#[cfg(test)]
pub mod tests {
    use crate::features::js_fetch::init;
    use hirofa_utils::js_utils::Script;
    use quickjs_runtime::builder::QuickJsRuntimeBuilder;

    //#[test]
    fn _test_fetch_generic() {
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
