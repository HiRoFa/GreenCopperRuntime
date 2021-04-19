use quickjs_runtime::eserror::EsError;
use quickjs_runtime::esvalue::EsValueFacade;
use std::future::Future;
use std::path::PathBuf;
use std::time::Duration;

pub struct GreenCopperFaas {}

// todo, can we share modules between contexts? or do we need to have one context with all modules..
// modules are not GCed in quickjs, we'll to actively kill runtimes every now and then if so

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
        _function_id: &str,
        _function_specs: Vec<String>,
        _args: Vec<EsValueFacade>,
    ) -> Option<Box<dyn Future<Output = Result<EsValueFacade, EsError>>>> {
        unimplemented!()
    }
}

// run with default lazy static faas
pub fn run(
    _function_id: &str,
    _function_specs: Vec<String>,
    _args: Vec<EsValueFacade>,
) -> Option<Box<dyn Future<Output = Result<EsValueFacade, EsError>>>> {
    unimplemented!()
}
