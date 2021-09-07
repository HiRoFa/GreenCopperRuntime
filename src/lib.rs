use hirofa_utils::js_utils::facades::JsRuntimeBuilder;
#[cfg(feature = "quickjs")]
use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;

#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

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

// todo, dit ombouwen, geen directe dep naar runtimes maar een JsRuntimeBuilder meegeven aan deze functie
// danwel gewoon features beschikbaar maken met install functie die builder accepteerd
// in quickjsruntimes kun je dan als test_dep een ref naar gc maken om console te installen in test rt

pub fn init_greco_rt<B: JsRuntimeBuilder>(builder: &mut B) {
    init_greco_rt2(builder, true, true, true)
}

pub fn init_greco_rt2<B: JsRuntimeBuilder>(
    builder: &mut B,
    preprocs: bool,
    features: bool,
    modules: bool,
) {
    if modules {
        modules::init(builder);
    }
    if features {
        features::init(builder);
    }
    if preprocs {
        preprocessors::init(builder);
    }
}

#[cfg(test)]
pub mod tests {

    use crate::features::js_console;
    use crate::preprocessors::cpp::CppPreProcessor;
    use backtrace::Backtrace;
    use hirofa_utils::js_utils::adapters::{JsRealmAdapter, JsValueAdapter};
    use hirofa_utils::js_utils::facades::values::JsValueFacade;
    use hirofa_utils::js_utils::facades::JsRuntimeFacade;
    use log::LevelFilter;
    use quickjs_runtime::builder::QuickJsRuntimeBuilder;
    use std::panic;

    fn init_abstract_inner<T: JsRuntimeFacade>(rt_facade: &T) {
        let res = rt_facade.js_loop_realm_sync(None, |_rt, ctx_adapter| {
            ctx_adapter.js_install_closure(
                &["com", "my_company"],
                "testFunction",
                |_runtime, realm, _this, args| {
                    // return 1234
                    let arg1 = &args[0].js_to_i32();
                    let arg2 = &args[1].js_to_i32();
                    realm.js_i32_create(arg1 * arg2 * 3)
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
        let args: Vec<JsValueFacade> = vec![JsValueFacade::new_i32(2), JsValueFacade::new_i32(4)];
        let res =
            rt_facade.js_function_invoke_sync(None, &["com", "my_company"], "testFunction", args);
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

        {
            println!("testing quickjs");
            let quickjs_builder = QuickJsRuntimeBuilder::new();
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

    pub fn init_test_greco_rt() -> impl JsRuntimeFacade {
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

        let mut builder = QuickJsRuntimeBuilder::new().script_pre_processor(CppPreProcessor::new());

        js_console::init(&mut builder);

        builder.build()
    }
}
