use hirofa_utils::js_utils::JsError;
use quickjs_runtime::esvalue::EsValueFacade;
use std::future::Future;
use std::path::PathBuf;
use std::time::Duration;

pub struct GreenCopperFaas {}

// todo, can we share modules between contexts? or do we need to have one context with all modules..
// modules are not GCed in quickjs, we'll to actively kill runtimes every now and then if so
// todo create a feature for this so it can be called from JS (should even be able to start a faas service for StarLight from quickjs and such

impl GreenCopperFaas {
    pub fn new(
        _def_folder: PathBuf,
        _rt_ct: usize,
        _max_function_count: usize,
        _max_idle_time: Duration,
    ) -> Self {
        Self {}
    }
    /// run a Faas function, returns None if function is not defined
    ///
    pub fn run(
        &self,
        _function_id: &str, // this is the main id of the faas function, e.g. handleRequest
        _function_specs: Vec<String>, // this can be used to specify conditions for determining which faas function to use, e.g. vec!["path=/home", "port=8080", "domain=localhost"]
        _args: Vec<EsValueFacade>,    // arguments to pass to the function
        _runtime_specs: (), // specify how the runtime should be built, e.g. quickjs/boa/starlight/spidermonkey / other builder opts
    ) -> Option<Box<dyn Future<Output = Result<EsValueFacade, JsError>>>> {
        unimplemented!()
    }
}

// run with default lazy static faas
pub fn run(
    _function_id: &str,
    _function_specs: Vec<String>,
    _args: Vec<EsValueFacade>,
    _runtime_specs: (),
) -> Option<Box<dyn Future<Output = Result<EsValueFacade, JsError>>>> {
    unimplemented!()
}
