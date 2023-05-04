use base64::Engine;
use quickjs_runtime::builder::QuickJsRuntimeBuilder;
use quickjs_runtime::jsutils::jsproxies::JsProxy;
use quickjs_runtime::jsutils::modules::NativeModuleLoader;
use quickjs_runtime::jsutils::JsError;
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use quickjs_runtime::quickjsvalueadapter::QuickJsValueAdapter;

struct EncodingModuleLoader {}

impl NativeModuleLoader for EncodingModuleLoader {
    fn has_module(&self, _realm: &QuickJsRealmAdapter, module_name: &str) -> bool {
        module_name.eq("greco://encoding")
    }

    fn get_module_export_names(
        &self,
        _realm: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<&str> {
        vec!["Base64"]
    }

    fn get_module_exports(
        &self,
        realm: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<(&str, QuickJsValueAdapter)> {
        init_exports(realm).expect("init encoding exports failed")
    }
}

pub(crate) fn init(builder: QuickJsRuntimeBuilder) -> QuickJsRuntimeBuilder {
    builder.js_native_module_loader(EncodingModuleLoader {})
}

fn init_exports(
    realm: &QuickJsRealmAdapter,
) -> Result<Vec<(&'static str, QuickJsValueAdapter)>, JsError> {
    let base64_proxy_class = create_base64_proxy(realm);
    let base64_proxy_class_res = realm.install_proxy(base64_proxy_class, false)?;

    Ok(vec![("Base64", base64_proxy_class_res)])
}

pub(crate) fn create_base64_proxy(_realm: &QuickJsRealmAdapter) -> JsProxy {
    JsProxy::new()
        .namespace(&["greco", "encoding"])
        .name("Base64")
        .static_method("encode", |_runtime, realm, args| {
            // todo async

            if args.is_empty() || !args[0].is_typed_array() {
                Err(JsError::new_str("encode expects a single type array arg"))
            } else {
                let bytes = realm.copy_typed_array_buffer(&args[0])?;
                realm.create_resolving_promise(
                    move || {
                        let engine = base64::engine::general_purpose::STANDARD;
                        let encoded = engine.encode(bytes);
                        Ok(encoded)
                    },
                    |realm, p_res| realm.create_string(p_res.as_str()),
                )
            }
        })
        .static_method("encodeSync", |_runtime, realm, args| {
            // todo async

            if args.is_empty() || !args[0].is_typed_array() {
                Err(JsError::new_str("encode expects a single type array arg"))
            } else {
                let bytes = realm.copy_typed_array_buffer(&args[0])?;
                let engine = base64::engine::general_purpose::STANDARD;
                let encoded = engine.encode(bytes);

                realm.create_string(encoded.as_str())
            }
        })
        .static_method("decode", |_runtime, realm, args| {
            // todo async

            if args.is_empty() || !args[0].is_string() {
                Err(JsError::new_str("decode expects a single string arg"))
            } else {
                let s = args[0].to_string()?;
                realm.create_resolving_promise(
                    move || {
                        let engine = base64::engine::general_purpose::STANDARD;
                        let decoded = engine
                            .decode(s)
                            .map_err(|e| JsError::new_string(format!("{e}")))?;
                        Ok(decoded)
                    },
                    |realm, p_res| {
                        //
                        realm.create_typed_array_uint8(p_res)
                    },
                )
            }
        })
        .static_method("decodeSync", |_runtime, realm, args| {
            // todo async

            if args.is_empty() || !args[0].is_string() {
                Err(JsError::new_str("decode expects a single string arg"))
            } else {
                let s = args[0].to_str()?;
                let engine = base64::engine::general_purpose::STANDARD;
                let decoded = engine
                    .decode(s)
                    .map_err(|e| JsError::new_string(format!("{e}")))?;
                //
                realm.create_typed_array_uint8(decoded)
            }
        })
}

#[cfg(test)]
pub mod tests {
    use futures::executor::block_on;
    //use log::LevelFilter;
    use quickjs_runtime::builder::QuickJsRuntimeBuilder;
    use quickjs_runtime::jsutils::Script;
    use quickjs_runtime::values::JsValueFacade;

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
        let res: JsValueFacade = block_on(rt.eval(None, script)).ok().expect("script failed");

        println!("{}", res.stringify());
        if let JsValueFacade::JsPromise { cached_promise } = res {
            let p_res = block_on(cached_promise.get_promise_result())
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
