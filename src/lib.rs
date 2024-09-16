use quickjs_runtime::builder::QuickJsRuntimeBuilder;

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
extern crate core;

#[cfg(any(
    feature = "all",
    feature = "features",
    feature = "console",
    feature = "fetch"
))]
pub mod features;

pub mod moduleloaders;

pub mod modules;
pub mod preprocessors;

pub fn init_greco_rt(builder: QuickJsRuntimeBuilder) -> QuickJsRuntimeBuilder {
    init_greco_rt2(builder, true, true, true)
}

pub fn init_greco_rt2(
    builder: QuickJsRuntimeBuilder,
    preprocs: bool,
    features: bool,
    modules: bool,
) -> QuickJsRuntimeBuilder {
    let mut builder = builder;
    if modules {
        builder = modules::init(builder);
    }
    if features {
        builder = features::init(builder);
    }
    if preprocs {
        builder = preprocessors::init(builder);
    }
    builder
}

#[cfg(test)]
pub mod tests {

    use crate::preprocessors::cpp::CppPreProcessor;
    use backtrace::Backtrace;
    use log::LevelFilter;
    use quickjs_runtime::builder::QuickJsRuntimeBuilder;
    use quickjs_runtime::facades::QuickJsRuntimeFacade;
    use quickjs_runtime::values::JsValueFacade;
    use std::panic;

    fn init_abstract_inner(rt_facade: &QuickJsRuntimeFacade) {
        let res = rt_facade.loop_realm_sync(None, |_rt, ctx_adapter| {
            ctx_adapter.install_closure(
                &["com", "my_company"],
                "testFunction",
                |_runtime, realm, _this, args| {
                    // return 1234
                    let arg1 = &args[0].to_i32();
                    let arg2 = &args[1].to_i32();
                    realm.create_i32(arg1 * arg2 * 3)
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

    fn test_abstract_inner(rt_facade: &QuickJsRuntimeFacade) {
        let args: Vec<JsValueFacade> = vec![JsValueFacade::new_i32(2), JsValueFacade::new_i32(4)];
        let res =
            rt_facade.invoke_function_sync(None, &["com", "my_company"], "testFunction", args);
        match res {
            Ok(val) => {
                if let JsValueFacade::I32 { val } = val {
                    assert_eq!(val, 2 * 4 * 3);
                } else {
                    panic!("script did not return a i32")
                }
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
            println!("thread panic occurred: {panic_info}\nbacktrace: {backtrace:?}");
            log::error!(
                "thread panic occurred: {}\nbacktrace: {:?}",
                panic_info,
                backtrace
            );
        }));

        simple_logging::log_to_file("greco_rt.log", LevelFilter::Trace)
            .ok()
            .unwrap();

        {
            println!("testing quickjs");
            let quickjs_builder = QuickJsRuntimeBuilder::new();
            //let builder1: JsRuntimeBuilder = quickjs_builder;
            let rt1 = quickjs_builder.build();
            init_abstract_inner(&rt1);
            test_abstract_inner(&rt1);
        }
    }

    #[test]
    fn test1() {
        let rt = init_test_greco_rt();
        drop(rt);
    }

    pub fn init_test_greco_rt() -> QuickJsRuntimeFacade {
        panic::set_hook(Box::new(|panic_info| {
            let backtrace = Backtrace::new();
            println!("thread panic occurred: {panic_info}\nbacktrace: {backtrace:?}");
            log::error!(
                "thread panic occurred: {}\nbacktrace: {:?}",
                panic_info,
                backtrace
            );
        }));

        simple_logging::log_to_file("greco_rt.log", LevelFilter::Trace)
            .ok()
            .unwrap();

        let builder = QuickJsRuntimeBuilder::new().script_pre_processor(CppPreProcessor::new());

        builder.build()
    }
}
