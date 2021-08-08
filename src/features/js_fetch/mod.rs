//! runtime agnostic fetch implementation

use hirofa_utils::js_utils::adapters::{JsRealmAdapter, JsRuntimeAdapter};
use hirofa_utils::js_utils::facades::{JsRuntimeBuilder, JsRuntimeFacade};
use hirofa_utils::js_utils::JsError;

pub mod spec;

pub fn init<T: JsRuntimeBuilder>(builder: &mut T) {
    // todo abstract trait for builders
    builder.js_runtime_init_hook(|rt| {
        rt.js_loop_sync(|rta| rta.js_add_realm_init_hook(|rt, realm| impl_for(realm)))
    });
}

pub fn impl_for<C>(ctx: &C) -> Result<(), JsError>
where
    C: JsRealmAdapter,
{
    ctx.js_install_function(
        &[],
        "fetch2",
        |js_rt, js_ctx, _this_obj, _args| {
            //
            js_ctx.js_null_create()
        },
        2,
    )
}

#[cfg(test)]
pub mod tests {
    use hirofa_utils::js_utils::Script;
    use quickjs_runtime::builder::QuickjsRuntimeBuilder;

    //#[test]
    fn test_fetch_generic() {
        let rt = QuickjsRuntimeBuilder::new().build();
        let res = rt.eval_sync(Script::new("test_fetch_gen.es", ""));
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
