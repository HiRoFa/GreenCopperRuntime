//! require
//!
//! this mod implements a require method which can be used to load CommonJS modules
//!
//! It uses the available ScriptModuleLoader instances in QuickJSRuntime
//!

use hirofa_utils::js_utils::adapters::{JsRealmAdapter, JsRuntimeAdapter, JsValueAdapter};
use hirofa_utils::js_utils::facades::{JsRuntimeBuilder, JsRuntimeFacade, JsValueType};
use hirofa_utils::js_utils::{JsError, Script};

pub fn init<T: JsRuntimeBuilder>(builder: &mut T) {
    // todo.. this should utilize the script module loaders in order to obtain the source, then use a 'require' function in js to do the actual loading..
    builder.js_runtime_init_hook(|rt| {
        // todo, impl with native function.. like now

        rt.js_loop_sync(|js_rt| {
            js_rt.js_add_realm_init_hook(|js_rt, realm| {
                //let global = get_global_q(q_ctx);
                //let require_func =
                //    new_native_function_q(q_ctx, "require", Some(require), 1, false)?;
                //set_property2_q(q_ctx, &global, "require", &require_func, 0)?;
                realm.js_install_function(&[], "require", require, 1)
            })
        })?;

        Ok(())
    });
}

const DEFAULT_EXTENSIONS: &[&str] = &["js", "mjs", "ts", "mts"];

fn require<T: JsRealmAdapter>(
    runtime: &T::JsRuntimeAdapterType,
    realm: &T,
    this_val: &<<T as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
    args: &[<<T as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType],
) -> Result<
    <<T as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
    JsError,
> {
    if args.len() != 1 || args[0].js_get_type() != JsValueType::String {
        Err(JsError::new_str(
            "require requires a single string argument",
        ))
    } else {
        let name = args[0].js_to_string()?;

        let mut cur_path = realm
            .js_get_script_or_module_name()
            .ok()
            .unwrap_or_else(|| "file:///node_modules/foo.js".to_string());

        // * if name does not start with / or ./ or ../ then use node_modules ref_path (if ref_path is file:///??)
        // todo , where do i cache these? a shutdown hook on a QuickJsContext would be nice to clear my own caches
        // much rather have a q_ctx.cache_region("").cache(id, obj)

        // see https://nodejs.org/en/knowledge/getting_started/what_is_require
        // * todo 2 support for directories, and then greco_jspreproc.js or package.json?

        // hmm if a module is loaded from https://somegit.somesite.com/scripts/kewlStuff.js and that does a require.. do we look in node_modules on disk?
        if !(name.contains("://")
            || name.starts_with("./")
            || name.starts_with("../")
            || name.starts_with('/'))
        {
            cur_path = format!("file:///node_modules/{}/foo.js", name);
        }

        log::debug!("require: {} -> {}", cur_path, name);

        let module_script_opt = (|| {
            let opt = runtime.js_load_module_script(cur_path.as_str(), name.as_str());
            if opt.is_some() {
                return opt;
            }
            for ext in DEFAULT_EXTENSIONS {
                let opt = runtime.js_load_module_script(
                    cur_path.as_str(),
                    format!("{}.{}", name.as_str(), ext).as_str(),
                );
                if opt.is_some() {
                    return opt;
                }
            }

            // see if index.js exists
            let mut base_name = name.clone();
            if let Some(rpos) = base_name.rfind('/') {
                let _ = base_name.split_off(rpos + 1);
            } else {
                base_name = "".to_string();
            }
            let opt = runtime.js_load_module_script(
                cur_path.as_str(),
                format!("{}{}", base_name, "index.js").as_str(),
            );
            if opt.is_some() {
                return opt;
            }

            None
        })();

        if let Some(module_script) = module_script_opt {
            // todo need to wrap as ES6 module so ScriptOrModuleName is sound for children
            log::debug!("found module script at {}", module_script.get_path());

            let wrapped_eval_code = format!(
                    "(function(){{const module = {{exports:{{}}}};let exports = module.exports;{{{}\n}}; return(module.exports);}}())",
                    module_script.get_code()
                );
            let eval_res = realm.js_eval(Script::new(
                module_script.get_path(),
                wrapped_eval_code.as_str(),
            ));
            eval_res
        } else {
            log::error!("module not found: {} -> {}", cur_path, name);
            Err(JsError::new_string(format!(
                "module not found: {} -> {}",
                cur_path, name
            )))
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_eval() {}
}
