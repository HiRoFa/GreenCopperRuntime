use hirofa_utils::js_utils::facades::JsRuntimeBuilder;
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

pub mod faas;

pub struct JsEngine {}
impl JsEngine {
    #[cfg(feature = "quickjs_runtime")]
    pub fn quickjs_builder() -> quickjs_runtime::esruntimebuilder::EsRuntimeBuilder {
        quickjs_runtime::esruntimebuilder::EsRuntimeBuilder::new()
    }
    #[cfg(feature = "starlight_runtime")]
    pub fn starlight_builder() -> impl JsRuntimeBuilder {
        starlight_runtime::slruntimefacade::StarlightRuntimeBuilder::new()
    }
}

pub fn new_greco_rt<R: JsRuntimeBuilder>(builder: R) -> R::JsRuntimeFacadeType {
    builder.build()
}

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
    use crate::{new_greco_rt_builder, JsEngine};

    use backtrace::Backtrace;
    use hirofa_utils::js_utils::adapters::{JsRealmAdapter, JsValueAdapter};
    use hirofa_utils::js_utils::facades::{JsRuntimeFacade, JsValueFacade, JsValueType};
    use log::LevelFilter;
    use quickjs_runtime::esruntime::EsRuntime;
    use std::panic;

    fn init_abstract_inner<T: JsRuntimeFacade>(rt_facade: &T) {
        let res = rt_facade.js_loop_realm_sync(None, |_rt, ctx_adapter| {
            ctx_adapter.js_install_closure(
                &["com", "my_company"],
                "testFunction",
                |ctx, _this, args| {
                    // return 1234
                    let arg1 = &args[0].js_to_i32();
                    let arg2 = &args[1].js_to_i32();
                    ctx.js_i32_create(arg1 * arg2 * 3)
                },
                2,
            )
        });
        match res {
            Ok(_) => {}
            Err(err) => {
                panic!("could not init: {}", err);
            }
        }
    }

    fn test_abstract_inner<T: JsRuntimeFacade>(rt_facade: &T) {
        let args: Vec<Box<dyn JsValueFacade>> = vec![Box::new(2), Box::new(4)];
        let res =
            rt_facade.js_function_invoke_sync(None, &["com", "my_company"], "testFunction", args);
        match res {
            Ok(val) => {
                assert!(val.js_get_type() == JsValueType::I32);
                assert_eq!(val.js_as_i32(), 2 * 4 * 3);
            }
            Err(err) => {
                panic!("func failed: {}", err);
            }
        }
    }

    #[test]
    fn test_abstract() {
        panic::set_hook(Box::new(|panic_info| {
            let backtrace = Backtrace::new();
            println!(
                "thread panic occurred: {}\nbacktrace: {:?}",
                panic_info, backtrace
            );
            log::error!(
                "thread panic occurred: {}\nbacktrace: {:?}",
                panic_info,
                backtrace
            );
        }));

        simple_logging::log_to_file("greco_rt.log", LevelFilter::Trace)
            .ok()
            .unwrap();

        #[cfg(feature = "quickjs_runtime")]
        {
            println!("testing quickjs");
            let quickjs_builder = JsEngine::quickjs_builder();
            //let builder1: JsRuntimeBuilder = quickjs_builder;
            let rt1 = quickjs_builder.build();
            init_abstract_inner(&rt1);
            test_abstract_inner(&rt1);
        }

        #[cfg(feature = "starlight_runtime")]
        {
            println!("testing starlight");
            let starlight_builder = JsEngine::starlight_builder();
            let rt2 = starlight_builder.build();
            init_abstract_inner(&rt2);
            test_abstract_inner(&rt2);
        }
    }

    #[test]
    fn test1() {
        let rt = init_test_greco_rt();
        drop(rt);
    }

    pub fn init_test_greco_rt() -> EsRuntime {
        panic::set_hook(Box::new(|panic_info| {
            let backtrace = Backtrace::new();
            println!(
                "thread panic occurred: {}\nbacktrace: {:?}",
                panic_info, backtrace
            );
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
