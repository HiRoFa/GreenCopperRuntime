use base64::Engine;
use hirofa_utils::js_utils::adapters::proxies::JsProxy;
use hirofa_utils::js_utils::adapters::{JsRealmAdapter, JsValueAdapter};
use hirofa_utils::js_utils::facades::JsRuntimeBuilder;
use hirofa_utils::js_utils::modules::NativeModuleLoader;
use hirofa_utils::js_utils::JsError;

struct EncodingModuleLoader {}

impl<R: JsRealmAdapter + 'static> NativeModuleLoader<R> for EncodingModuleLoader {
    fn has_module(&self, _realm: &R, module_name: &str) -> bool {
        module_name.eq("greco://encoding")
    }

    fn get_module_export_names(&self, _realm: &R, _module_name: &str) -> Vec<&str> {
        vec!["Base64"]
    }

    fn get_module_exports(
        &self,
        realm: &R,
        _module_name: &str,
    ) -> Vec<(&str, R::JsValueAdapterType)> {
        init_exports(realm).expect("init encoding exports failed")
    }
}

pub(crate) fn init<B: JsRuntimeBuilder>(builder: B) -> B {
    builder.js_native_module_loader(EncodingModuleLoader {})
}

fn init_exports<R: JsRealmAdapter + 'static>(
    realm: &R,
) -> Result<Vec<(&'static str, R::JsValueAdapterType)>, JsError> {
    let base64_proxy_class = create_base64_proxy(realm);
    let base64_proxy_class_res = realm.js_proxy_install(base64_proxy_class, false)?;

    Ok(vec![("Base64", base64_proxy_class_res)])
}

pub(crate) fn create_base64_proxy<R: JsRealmAdapter + 'static>(_realm: &R) -> JsProxy<R> {
    JsProxy::new(&["greco", "encoding"], "Base64")
        .add_static_method("encode", |_runtime, realm: &R, args| {
            // todo async

            if args.is_empty() || !args[0].js_is_typed_array() {
                Err(JsError::new_str("encode expects a single type array arg"))
            } else {
                let bytes = realm.js_typed_array_copy_buffer(&args[0])?;
                realm.js_promise_create_resolving(
                    move || {
                        let engine = base64::engine::general_purpose::STANDARD;
                        let encoded = engine.encode(bytes);
                        Ok(encoded)
                    },
                    |realm, p_res| realm.js_string_create(p_res.as_str()),
                )
            }
        })
        .add_static_method("encodeSync", |_runtime, realm: &R, args| {
            // todo async

            if args.is_empty() || !args[0].js_is_typed_array() {
                Err(JsError::new_str("encode expects a single type array arg"))
            } else {
                let bytes = realm.js_typed_array_copy_buffer(&args[0])?;
                let engine = base64::engine::general_purpose::STANDARD;
                let encoded = engine.encode(bytes);

                realm.js_string_create(encoded.as_str())
            }
        })
        .add_static_method("decode", |_runtime, realm: &R, args| {
            // todo async

            if args.is_empty() || !args[0].js_is_string() {
                Err(JsError::new_str("decode expects a single string arg"))
            } else {
                let s = args[0].js_to_string()?;
                realm.js_promise_create_resolving(
                    move || {
                        let engine = base64::engine::general_purpose::STANDARD;
                        let decoded = engine
                            .decode(s)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;
                        Ok(decoded)
                    },
                    |realm, p_res| {
                        //
                        realm.js_typed_array_uint8_create(p_res)
                    },
                )
            }
        })
        .add_static_method("decodeSync", |_runtime, realm: &R, args| {
            // todo async

            if args.is_empty() || !args[0].js_is_string() {
                Err(JsError::new_str("decode expects a single string arg"))
            } else {
                let s = args[0].js_to_str()?;
                let engine = base64::engine::general_purpose::STANDARD;
                let decoded = engine
                    .decode(s)
                    .map_err(|e| JsError::new_string(format!("{e}")))?;
                //
                realm.js_typed_array_uint8_create(decoded)
            }
        })
}

#[cfg(test)]
pub mod tests {
    use futures::executor::block_on;
    use hirofa_utils::js_utils::facades::values::JsValueFacade;
    use hirofa_utils::js_utils::facades::JsRuntimeFacade;
    use hirofa_utils::js_utils::Script;
    //use log::LevelFilter;
    use quickjs_runtime::builder::QuickJsRuntimeBuilder;

    #[test]
    fn test_b64() {
        //simple_logging::log_to_stderr(log::LevelFilter::Info);

        let builder = QuickJsRuntimeBuilder::new();
        let builder = crate::init_greco_rt(builder);
        let rt = builder.build();

        let script = Script::new(
            "test_encoding.js",
            r#"

        async function test() {
            let encodingMod = await import('greco://encoding');

            let data = 'QUJDRA==';
            //Uint8Array(4) [ 65, 66, 67, 68 ]

            let arr = await encodingMod.Base64.decode(data);
            let b64 = await encodingMod.Base64.encode(arr);
            arr = encodingMod.Base64.decodeSync(b64);
            b64 = encodingMod.Base64.encodeSync(arr);
                      
            return b64;

        }

        test()

        "#,
        );
        let res: JsValueFacade = block_on(rt.js_eval(None, script))
            .ok()
            .expect("script failed");

        println!("{}", res.stringify());
        if let JsValueFacade::JsPromise { cached_promise } = res {
            let rti_weak = rt.js_get_runtime_facade_inner();
            let rti = rti_weak.upgrade().expect("invalid state");
            let p_res = block_on(cached_promise.js_get_promise_result(&*rti))
                .ok()
                .expect("get prom res failed");
            match p_res {
                Ok(jsvf) => {
                    println!("prom resolved to {}", jsvf.stringify());
                }
                Err(e) => {
                    panic!("prom rejected: {}", e.stringify());
                }
            }
        } else {
            panic!("did not get a promise");
        }
    }
}
