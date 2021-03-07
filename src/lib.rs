use quickjs_runtime::esruntime::EsRuntime;
use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;
use quickjs_runtime::esscript::EsScript;
use quickjs_runtime::features::fetch::response::FetchResponse;
use quickjs_runtime::quickjscontext::QuickJsContext;
use quickjs_runtime::quickjsruntime::{ModuleScriptLoader, NativeModuleLoader};
use std::sync::Arc;

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

mod features;

pub struct GrecoRuntimeBuilder {
    fetch_response_provider: Box<dyn FetchResponseProvider>,
    module_script_loaders: Vec<Box<ModuleScriptLoader>>,
    native_module_loaders: Vec<Box<dyn NativeModuleLoader>>,
}

impl GrecoRuntimeBuilder {
    pub fn new() -> Self {
        Self {
            fetch_response_provider: Box::new(()),
            module_script_loaders: vec![],
            native_module_loaders: vec![],
        }
    }

    // should have methods to add extra module loaders (native and script) these are passed as a multiloader to EsRuntime
    // todo EsRuntimeBuilder should fail on adding multiple loaders

    pub fn build(self) -> Arc<EsRuntime> {
        // todo config, add extra module loaders / script laoders etc
        // gc server will want a custom loader for both files and libs
        // pass those as param here? in Option? or impl as builder? builder pelase

        // add fetch here
        let rt = EsRuntimeBuilder::new()
            //.fetch_response_provider(todo)
            //.native_module_loader(todo)
            //.module_script_loader(todo)
            .build();

        match rt.add_to_event_queue_sync(|q_js_rt| {
            q_js_rt.add_context_init_hook(|q_js_rt, q_ctx| {
                // this is only for adding toplevel methods like Worker,setTimeout etc
                // all features should be dynamically loaded through the native module loader
                features::init(q_js_rt, q_ctx)
            })
        }) {
            Ok(_) => {}
            Err(e) => panic!("init failed: {}", e),
        }
        rt
    }

    pub fn script_module_loader<L>(mut self, loader: L) -> Self
    where
        L: Fn(&QuickJsContext, &str, &str) -> Option<EsScript> + Send + 'static,
    {
        self.module_script_loaders.push(loader);
        self
    }
    pub fn native_module_loader<L>(mut self, loader: L) -> Self
    where
        L: NativeModuleLoader + Send + 'static,
    {
        self.native_module_loaders.push(loader);
        self
    }
}

#[cfg(test)]
pub mod tests {
    use crate::GrecoRuntimeBuilder;

    use log::LevelFilter;
    use quickjs_runtime::esruntime::EsRuntime;
    use std::sync::Arc;

    lazy_static! {
        pub static ref TEST_RT: Arc<EsRuntime> = init_rt();
    }

    fn init_rt() -> Arc<EsRuntime> {
        simple_logging::log_to_file("greco_rt.log", LevelFilter::Trace)
            .ok()
            .unwrap();

        GrecoRuntimeBuilder::new().build()
    }
}
