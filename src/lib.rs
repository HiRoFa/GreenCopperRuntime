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
    let mut rt_builder = EsRuntimeBuilder::new();

    rt_builder = modules::init(rt_builder);
    rt_builder = features::init(rt_builder);
    rt_builder = preprocessors::init(rt_builder);

    rt_builder
}

#[cfg(test)]
pub mod tests {
    use crate::new_greco_rt_builder;

    use log::LevelFilter;
    use quickjs_runtime::esruntime::EsRuntime;
    use std::sync::Arc;

    #[test]
    fn test1() {
        let rt = init_test_greco_rt();
        drop(rt);
    }

    pub fn init_test_greco_rt() -> Arc<EsRuntime> {
        simple_logging::log_to_file("greco_rt.log", LevelFilter::Trace)
            .ok()
            .unwrap();

        new_greco_rt_builder().build()
    }
}
