//! require
//!
//! this mod implements a require method which can be used to load CommonJS modules
//!
//! It uses the available ScriptModuleLoader instances in QuickJSRuntime
//!

use libquickjs_sys as q;
use quickjs_runtime::esruntime::EsRuntime;
use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;
use quickjs_runtime::quickjs_utils::functions::new_native_function_q;
use quickjs_runtime::quickjs_utils::objects::set_property2_q;
use quickjs_runtime::quickjs_utils::{get_global_q, parse_args, primitives};
use quickjs_runtime::quickjsruntime::QuickJsRuntime;

pub(crate) fn init(builder: EsRuntimeBuilder) -> EsRuntimeBuilder {
    // todo.. this should utilize the script module loaders in order to obtain the source, then use a 'require' function in js to do the actual loading..
    builder.runtime_init_hook(|rt: &EsRuntime| {
        // todo, impl with native function.. like now

        rt.add_to_event_queue_sync(|q_js_rt| {
            q_js_rt.add_context_init_hook(|_q_js_rt, q_ctx| {
                let global = get_global_q(q_ctx);
                let require_func =
                    new_native_function_q(q_ctx, "require", Some(require), 1, false)?;
                set_property2_q(q_ctx, &global, "require", &require_func, 0)?;
                Ok(())
            })
        })?;

        Ok(())
    })
}

unsafe extern "C" fn require(
    context: *mut q::JSContext,
    _this_val: q::JSValue,
    argc: ::std::os::raw::c_int,
    argv: *mut q::JSValue,
) -> q::JSValue {
    let args = parse_args(context, argc, argv);
    QuickJsRuntime::do_with(|q_js_rt| {
        let q_ctx = q_js_rt.get_quickjs_context(context);
        if args.len() != 1 || !args[0].is_string() {
            q_ctx.report_ex("require requires a single string argument")
        } else {
            let name = primitives::to_string_q(q_ctx, &args[0])
                .ok()
                .expect("to_string failed");

            // todo instead of node_modules get current path using JS_GetScriptOrModuleName
            if let Some(module_script) =
                q_js_rt.load_module_script_opt("file:///node_modules/foo.js", name.as_str())
            {
                let wrapped_function_code = format!(
                    "function(){{let exports = {{}};{{\n{}\n}} return exports;}};",
                    module_script.as_str()
                );

                let func_res = quickjs_runtime::quickjs_utils::functions::parse_function(
                    q_ctx.context,
                    false,
                    "require_wrapper",
                    wrapped_function_code.as_str(),
                    vec![],
                );
                match func_res {
                    Ok(func) => {
                        let exe_res = quickjs_runtime::quickjs_utils::functions::call_function_q(
                            q_ctx,
                            &func,
                            vec![],
                            None,
                        );
                        match exe_res {
                            Ok(export_obj) => export_obj.clone_value_incr_rc(),
                            Err(e) => {
                                q_ctx.report_ex(format!("Module invocation failed: {}", e).as_str())
                            }
                        }
                    }
                    Err(e) => q_ctx.report_ex(format!("Module parsing failed: {}", e).as_str()),
                }
            } else {
                q_ctx.report_ex("module not found")
            }
            // wip
        }
    })
}
