//! runtime agnostic fetch implementation

use hirofa_utils::js_utils::adapters::JsRealmAdapter;
use hirofa_utils::js_utils::JsError;
use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;

pub(crate) fn init(builder: EsRuntimeBuilder) -> EsRuntimeBuilder {
    // todo abstract trait for builders
    builder.runtime_init_hook(|rt| {
        rt.exe_rt_task_in_event_loop(|qjs_rt| {
            qjs_rt.add_context_init_hook(|_qjs_rt, ctx| impl_for(ctx))
        })
    })
}

pub fn impl_for<C>(ctx: &C) -> Result<(), JsError>
where
    C: JsRealmAdapter,
{
    ctx.js_install_function(
        &[],
        "fetch2",
        |js_ctx, _this_obj, _args| {
            //
            js_ctx.js_null_create()
        },
        2,
    )
}

#[cfg(test)]
pub mod tests {
    use crate::new_greco_rt_builder;
    use hirofa_utils::js_utils::Script;

    //#[test]
    fn test_fetch_generic() {
        let rt = new_greco_rt_builder().build();
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
