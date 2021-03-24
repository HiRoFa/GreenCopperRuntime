use quickjs_runtime::eserror::EsError;
use quickjs_runtime::esruntime::EsRuntime;
use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;
use quickjs_runtime::esvalue::{EsNullValue, EsValueConvertible};
use quickjs_runtime::quickjsruntime::QuickJsRuntime;

pub(crate) fn init(builder: EsRuntimeBuilder) -> EsRuntimeBuilder {
    // todo.. this should utilize the script module loaders in order to obtain the source, then use a 'require' function in js to do the actual loading..
    builder.runtime_init_hook(|rt: &EsRuntime| {
        // todo
        rt.set_function(vec![], "require", |_q_ctx, args| {
            if args.len() != 1 || !args[0].is_string() {
                Err(EsError::new_str(
                    "require requires a single string argument",
                ))
            } else {
                QuickJsRuntime::do_with(|q_js_rt| {
                    //q_js_rt.module_loaders... haha i'm not in the effing cratejfjfgvhjdj
                    // wip
                });

                Ok(EsNullValue {}.to_es_value_facade())
            }
        })
    })
}
