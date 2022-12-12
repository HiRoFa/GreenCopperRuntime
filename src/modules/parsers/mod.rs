use csv::Trim;
use hirofa_utils::js_utils::adapters::proxies::JsProxy;
use hirofa_utils::js_utils::adapters::{JsRealmAdapter, JsValueAdapter};
use hirofa_utils::js_utils::facades::values::JsValueFacade;
use hirofa_utils::js_utils::facades::JsRuntimeBuilder;
use hirofa_utils::js_utils::modules::NativeModuleLoader;
use hirofa_utils::js_utils::JsError;
use std::str;

struct ParsersModuleLoader {}

impl<R: JsRealmAdapter + 'static> NativeModuleLoader<R> for ParsersModuleLoader {
    fn has_module(&self, _realm: &R, module_name: &str) -> bool {
        module_name.eq("greco://parsers")
    }

    fn get_module_export_names(&self, _realm: &R, _module_name: &str) -> Vec<&str> {
        vec!["CsvParser"]
    }

    fn get_module_exports(
        &self,
        realm: &R,
        _module_name: &str,
    ) -> Vec<(&str, R::JsValueAdapterType)> {
        init_exports(realm)
            .expect("init parsers exports failed")
    }
}

pub(crate) fn init<B: JsRuntimeBuilder>(builder: B) -> B {
    builder.js_native_module_loader(ParsersModuleLoader {})
}

fn init_exports<R: JsRealmAdapter + 'static>(
    realm: &R,
) -> Result<Vec<(&'static str, R::JsValueAdapterType)>, JsError> {
    let csv_parser_proxy_class = create_csv_parser_proxy(realm);
    let csv_parser_res = realm.js_proxy_install(csv_parser_proxy_class, false)?;

    Ok(vec![("CsvParser", csv_parser_res)])
}

pub(crate) fn create_csv_parser_proxy<R: JsRealmAdapter + 'static>(_realm: &R) -> JsProxy<R> {
    JsProxy::new(&["greco", "parsers"], "CsvParser")
        .add_static_method("parse", |_runtime, realm: &R,  args| {

            // three args, a string or Uint8array for data, a recordCallBack function and an optional options object

            if args.len() < 3 || !(args[0].js_is_string() || args[0].js_is_typed_array()) || !args[1].js_is_function() || !args[2].js_is_function() {
                Err(JsError::new_str("parse requires 2 or 3 args (data: string | Uint8Array, headersCallBack: (headers: array<string>) => void, recordCallback: (record: array<string>) => void, options: {})"))
            } else {

                // get data, func_ref as JsValueFacade, move to producer

                // convert vec<u8> to string here
                // todo move bytes and read direct instead of to string first (require Either i guess)
                let data = if args[0].js_is_string() {
                    args[0].js_to_string()?
                } else {
                    let buf = realm.js_typed_array_detach_buffer(&args[0])?;
                    str::from_utf8(&buf).map_err(|e| JsError::new_string(format!("{}", e)))?.to_string()
                };
                let cb_h_func = realm.to_js_value_facade(&args[1])?;
                let cb_r_func = realm.to_js_value_facade(&args[2])?;

                let rti_weak = realm.js_get_runtime_facade_inner();

                realm.js_promise_create_resolving_async(  async move {
                    //

                    let rti = match rti_weak.upgrade() {
                        None => {
                            Err(JsError::new_str("invalid state"))
                        }
                        Some(a) => Ok(a)
                    }?;

                    let mut rdr = csv::ReaderBuilder::new()
                        .double_quote(true)
                        .delimiter(b',')
                        .has_headers(true)
                        .quoting(true)
                        .flexible(true)
                        //.ascii()
                        .trim(Trim::All)
                        .from_reader(data.as_bytes());

                    let cached_h_function = if let JsValueFacade::JsFunction { cached_function } = cb_h_func { cached_function } else { panic!("function was not a function") };
                    let cached_r_function = if let JsValueFacade::JsFunction { cached_function } = cb_r_func { cached_function } else { panic!("function was not a function") };

                    let headers = rdr.headers().map_err(|e| JsError::new_string(format!("{}", e)))?;

                    log::trace!("greco::parsers::CsvParser headers: {:?}", headers);

                    let val: Vec<JsValueFacade> = headers.iter().map(|h| {
                        JsValueFacade::new_str(h)
                    }).collect();

                    let _ = cached_h_function.js_invoke_function(&*rti, vec![JsValueFacade::Array {val}]).await;

                    for result in rdr.records() {
                        // The iterator yields Result<StringRecord, Error>, so we check the
                        // error here.
                        let record = result.map_err(|e| JsError::new_string(format!("{}", e)))?;

                        // fill val from record
                        let val: Vec<JsValueFacade> = record.iter().map(|h| {
                            JsValueFacade::new_str(h)
                        }).collect();

                        let jsvf_record = JsValueFacade::Array {val};

                        let _ = cached_r_function.js_invoke_function(&*rti, vec![jsvf_record]).await;

                        log::trace!("greco::parsers::CsvParser row: {:?}", record);
                    }


                    Ok(())
                }, |realm, _result| {
                    realm.js_null_create()
                })
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
    fn test_csv() {
        //simple_logging::log_to_stderr(log::LevelFilter::Info);

        let builder = QuickJsRuntimeBuilder::new();
        let builder = crate::init_greco_rt(builder);
        let rt = builder.build();


        let script = Script::new(
            "test_parsers.js",
            r#"

        async function test() {
            let parsersMod = await import('greco://parsers');

            let data = '"r1", "r2", "r3", "r4"\n"a", "b", 1, 2\n"c", "d", 3, 4';

            await parsersMod.CsvParser.parse(data, (headers) => {
                console.log("headers: " + headers.join("-"));
            }, (row) => {
                console.log("row: " + row.join("-"));
            });
            console.log("parser done");

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
