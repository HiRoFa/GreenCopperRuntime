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
use quickjs_runtime::quickjs_utils::{
    get_global_q, get_script_or_module_name, parse_args, primitives,
};
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

            let mut cur_path = get_script_or_module_name(context)
                .ok()
                .unwrap_or_else(|| "file:///node_modules/foo.js".to_string());

            // * if name does not start with / or ./ or ../ then use node_modules ref_path (if ref_path is file:///??)
            // todo , where do i cache these? a shutdown hook on a QuickjsContext would be nice to clear my own caches
            // see https://nodejs.org/en/knowledge/getting_started/what_is_require
            // * todo 2 support for directories, and then index.js or package.json?

            // hmm if a module is loaded from https://somegit.somesite.com/scripts/kewlStuff.js and that does a require.. do we look in node_modules on disk?
            if cur_path.starts_with("file:///") {
                if !(name.starts_with("./") || name.starts_with("../") || name.starts_with("/")) {
                    cur_path = "file:///node_modules/foo.js".to_string();
                }
            }

            if let Some(module_script) =
                q_js_rt.load_module_script_opt(cur_path.as_str(), name.as_str())
            {
                let wrapped_function_code = format!(
                    "function(){{const module = {{exports:{{}}}};let exports = module.exports;{{\n{}\n}} return module.exports;}};",
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
