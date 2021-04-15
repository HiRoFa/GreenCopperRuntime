use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

pub mod features;
#[cfg(any(feature = "all", feature = "com", feature = "http"))]
pub mod fetch;

pub mod moduleloaders;
pub mod modules;
pub mod preprocessors;

pub fn new_greco_rt_builder() -> EsRuntimeBuilder {
    new_greco_rt_builder2(true, true, true)
}

pub fn new_greco_rt_builder2(preprocs: bool, features: bool, modules: bool) -> EsRuntimeBuilder {
    let mut rt_builder = EsRuntimeBuilder::new();
    if modules {
        rt_builder = modules::init(rt_builder);
    }
    if features {
        rt_builder = features::init(rt_builder);
    }
    if preprocs {
        rt_builder = preprocessors::init(rt_builder);
    }

    rt_builder
}

#[cfg(test)]
pub mod tests {
    use crate::new_greco_rt_builder;

    use backtrace::Backtrace;
    use log::LevelFilter;
    use quickjs_runtime::esruntime::EsRuntime;
    use std::panic;
    use std::sync::Arc;

    #[test]
    fn test1() {
        let rt = init_test_greco_rt();
        drop(rt);
    }

    pub fn init_test_greco_rt() -> Arc<EsRuntime> {
        panic::set_hook(Box::new(|panic_info| {
            let backtrace = Backtrace::new();
            log::error!(
                "thread panic occurred: {}\nbacktrace: {:?}",
                panic_info,
                backtrace
            );
        }));

        simple_logging::log_to_file("greco_rt.log", LevelFilter::Trace)
            .ok()
            .unwrap();

        new_greco_rt_builder().build()
    }
}
