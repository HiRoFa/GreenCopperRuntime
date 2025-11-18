use csv::Trim;
use quickjs_runtime::builder::QuickJsRuntimeBuilder;
use quickjs_runtime::jsutils::jsproxies::JsProxy;
use quickjs_runtime::jsutils::modules::NativeModuleLoader;
use quickjs_runtime::jsutils::JsError;
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use quickjs_runtime::quickjsvalueadapter::QuickJsValueAdapter;
use quickjs_runtime::values::JsValueFacade;
use std::str;

struct ParsersModuleLoader {}

impl NativeModuleLoader for ParsersModuleLoader {
    fn has_module(&self, _realm: &QuickJsRealmAdapter, module_name: &str) -> bool {
        module_name.eq("greco://parsers")
    }

    fn get_module_export_names(
        &self,
        _realm: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<&str> {
        vec!["CsvParser"]
    }

    fn get_module_exports(
        &self,
        realm: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<(&str, QuickJsValueAdapter)> {
        init_exports(realm).expect("init parsers exports failed")
    }
}

pub(crate) fn init(builder: QuickJsRuntimeBuilder) -> QuickJsRuntimeBuilder {
    builder.native_module_loader(ParsersModuleLoader {})
}

fn init_exports(
    realm: &QuickJsRealmAdapter,
) -> Result<Vec<(&'static str, QuickJsValueAdapter)>, JsError> {
    let csv_parser_proxy_class = create_csv_parser_proxy(realm);
    let csv_parser_res = realm.install_proxy(csv_parser_proxy_class, false)?;

    Ok(vec![("CsvParser", csv_parser_res)])
}

pub(crate) fn create_csv_parser_proxy(_realm: &QuickJsRealmAdapter) -> JsProxy {
    JsProxy::new().namespace(&["greco", "parsers"]).name("CsvParser")
        .static_method("parse", |_runtime, realm,  args| {

            // three args, a string or Uint8array for data, a recordCallBack function and an optional options object

            if args.len() < 3 || !(args[0].is_string() || args[0].is_typed_array()) || !args[1].is_function() || !args[2].is_function() {
                Err(JsError::new_str("parse requires 2 or 3 args (data: string | Uint8Array, headersCallBack: (headers: array<string>) => void, recordCallback: (record: array<string>) => void, options: {})"))
            } else {

                // get data, func_ref as JsValueFacade, move to producer

                let data = if args[0].is_string() {
                    args[0].to_string()?
                } else {
                    let buf = realm.copy_typed_array_buffer(&args[0])?;
                    String::from_utf8(buf).map_err(|e| JsError::new_string(format!("{e:?}")))?
                };
                let cb_h_func = realm.to_js_value_facade(&args[1])?;
                let cb_r_func = realm.to_js_value_facade(&args[2])?;

                realm.create_resolving_promise_async(  async move {

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

                    let headers = rdr.headers().map_err(|e| JsError::new_string(format!("{e:?}")))?;

                    log::trace!("greco::parsers::CsvParser headers: {:?}", headers);

                    let val: Vec<JsValueFacade> = headers.iter().map(|h| {
                        JsValueFacade::new_str(h)
                    }).collect();

                    let _ = cached_h_function.invoke_function( vec![JsValueFacade::Array {val}]).await;

                    for result in rdr.records() {
                        // The iterator yields Result<StringRecord, Error>, so we check the
                        // error here.
                        let record = result.map_err(|e| JsError::new_string(format!("{e:?}")))?;

                        // fill val from record
                        let val: Vec<JsValueFacade> = record.iter().map(|h| {
                            JsValueFacade::new_str(h)
                        }).collect();

                        let jsvf_record = JsValueFacade::Array {val};

                        let _ = cached_r_function.invoke_function( vec![jsvf_record]).await;

                        log::trace!("greco::parsers::CsvParser row: {:?}", record);
                    }


                    Ok(())
                }, |realm, _result| {
                    realm.create_null()
                })
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

            let data = '"r 1", "r2", "r3", "r4"\n"a", "b", 1, 2\n"c", "d", 3, 4';

            let ret = "";

            await parsersMod.CsvParser.parse(data, (headers) => {
                console.log("headers: " + headers.join("-"));
                ret += "headers: " + headers.join("-") + "\n";
            }, (row) => {
                console.log("row: " + row.join("-"));
                ret += "row: " + row.join("-") + "\n"

            });
            console.log("parser done");

            return ret;

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
