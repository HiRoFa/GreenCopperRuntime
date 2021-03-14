use quickjs_runtime::eserror::EsError;
use quickjs_runtime::esruntime::EsRuntime;
use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;
use quickjs_runtime::esscript::EsScript;
use quickjs_runtime::features::fetch::request::FetchRequest;
use quickjs_runtime::features::fetch::response::FetchResponse;
use quickjs_runtime::quickjscontext::QuickJsContext;
use quickjs_runtime::quickjsruntime::{NativeModuleLoader, ScriptModuleLoader};
use std::sync::Arc;

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

mod features;
mod moduleloaders;
mod modules;

pub struct GrecoRuntimeBuilder {}

impl GrecoRuntimeBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(self) -> Result<Arc<EsRuntime>, EsError> {
        // todo config, add extra module loaders / script loaders etc
        // gc server will want a custom loader for both files and libs
        // pass those as param here? in Option? or impl as builder? builder pelase

        let mut rt_builder = EsRuntimeBuilder::new();

        rt_builder = modules::init(rt_builder);

        // add fetch here

        let rt = rt_builder
            //.fetch_response_provider(todo)
            .build();

        rt.add_to_event_queue_sync(|q_js_rt| {
            q_js_rt.add_context_init_hook(|q_js_rt, q_ctx| {
                // this is only for adding toplevel methods like Worker,setTimeout etc
                // all features should be dynamically loaded through the native module loader
                features::init(q_js_rt, q_ctx)
            })
        })?;
        Ok(rt)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::GrecoRuntimeBuilder;

    use log::LevelFilter;
    use quickjs_runtime::eserror::EsError;
    use quickjs_runtime::esruntime::EsRuntime;
    use std::sync::Arc;

    lazy_static! {
        pub static ref TEST_RT: Arc<EsRuntime> = init_rt().ok().expect("init failed");
    }

    fn init_rt() -> Result<Arc<EsRuntime>, EsError> {
        simple_logging::log_to_file("greco_rt.log", LevelFilter::Trace)
            .ok()
            .unwrap();

        GrecoRuntimeBuilder::new().build()
    }
}
