//! the console feature enables the script to use various cansole.log variants
//! see also: [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Console)
//! the following methods are available
//! * console.log()
//! * console.info()
//! * console.error()
//! * console.warning()
//! * console.trace()
//!
//! The methods use rust's log crate to output messages. e.g. console.info() uses the log::info!() macro
//! so the console messages should appear in the log you initialized from rust
//!
//! All methods accept a single message string and optional substitution values
//!
//! e.g.
//! ```javascript
//! console.log('Oh dear %s totaly failed %i times because of a %.4f variance in the space time continuum', 'some guy', 12, 2.46)
//! ```
//! will output 'Oh dear some guy totaly failed 12 times because of a 2.4600 variance in the space time continuum'
//!
//! The string substitution you can use are
//! * %o or %O Outputs a JavaScript object (serialized)
//! * %d or %i Outputs an integer. Number formatting is supported, for example  console.log("Foo %.2d", 1.1) will output the number as two significant figures with a leading 0: Foo 01
//! * %s Outputs a string (will attempt to call .toString() on objects, use %o to output a serialized JSON string)
//! * %f Outputs a floating-point value. Formatting is supported, for example  console.log("Foo %.2f", 1.1) will output the number to 2 decimal places: Foo 1.10
//! # Example
//! ```rust
//! use hirofa_utils::js_utils::Script;
//! use log::LevelFilter;
//! use quickjs_runtime::builder::QuickJsRuntimeBuilder;
//! use hirofa_utils::js_utils::facades::JsRuntimeFacade;
//! simple_logging::log_to_file("console_test.log", LevelFilter::max())
//!             .ok()
//!             .expect("could not init logger");
//! let rt = QuickJsRuntimeBuilder::new().build();
//! rt.js_loop_sync_mut(|rta| {
//!     crate::green_copper_runtime::features::js_console::install_runtime(rta);
//! });
//!
//! rt.eval_sync(Script::new(
//! "console.es",
//! "console.log('the %s %s %s jumped over %i fences with a accuracy of %.2f', 'quick', 'brown', 'fox', 32, 0.512);"
//! ));
//! ```
//!
//! which will result in a log entry like
//! ```[00:00:00.012] (7f44e7d24700) INFO   the quick brown fox jumped over 32 fences with a accuracy of 0.51```

use hirofa_utils::js_utils::adapters::{JsRealmAdapter, JsRuntimeAdapter, JsValueAdapter};
use hirofa_utils::js_utils::facades::{JsRuntimeBuilder, JsRuntimeFacade, JsValueType};
use hirofa_utils::js_utils::JsError;
use log::LevelFilter;
use std::str::FromStr;

pub fn init<T: JsRuntimeBuilder>(builder: T) -> T {
    builder.js_runtime_init_hook(|rt| rt.js_loop_sync_mut(|rta| install_runtime(rta)))
}

pub fn install_runtime<R: JsRuntimeAdapter>(runtime_adapter: &mut R) -> Result<(), JsError> {
    runtime_adapter.js_add_realm_init_hook(|_, realm| install_realm(realm))
}

pub fn install_realm<R: JsRealmAdapter>(realm_adapter: &R) -> Result<(), JsError> {
    realm_adapter.js_install_function(&["console"], "log", console_log, 1)?;
    realm_adapter.js_install_function(&["console"], "info", console_info, 1)?;
    realm_adapter.js_install_function(&["console"], "debug", console_debug, 1)?;
    realm_adapter.js_install_function(&["console"], "warn", console_warn, 1)?;
    realm_adapter.js_install_function(&["console"], "error", console_error, 1)?;
    realm_adapter.js_install_function(&["console"], "trace", console_trace, 1)?;

    Ok(())
}

fn stringify_log_obj<R: JsRealmAdapter>(realm: &R, arg: &R::JsValueAdapterType) -> String {
    match realm.js_json_stringify(arg, None) {
        Ok(r) => r,
        Err(e) => format!("Error: {}", e),
    }
}

fn console_log<R: JsRealmAdapter>(
    _runtime_adapter: &R::JsRuntimeAdapterType,
    realm_adapter: &R,
    _this_arg: &R::JsValueAdapterType,
    args: &[R::JsValueAdapterType],
) -> Result<R::JsValueAdapterType, JsError> {
    if log::max_level() >= LevelFilter::Info {
        log::info!("{}", parse_line(realm_adapter, args));
    }

    realm_adapter.js_null_create()
}

fn console_info<R: JsRealmAdapter>(
    _runtime_adapter: &R::JsRuntimeAdapterType,
    realm_adapter: &R,
    _this_arg: &R::JsValueAdapterType,
    args: &[R::JsValueAdapterType],
) -> Result<R::JsValueAdapterType, JsError> {
    if log::max_level() >= LevelFilter::Info {
        log::info!("{}", parse_line(realm_adapter, args));
    }

    realm_adapter.js_null_create()
}

fn console_error<R: JsRealmAdapter>(
    _runtime_adapter: &R::JsRuntimeAdapterType,
    realm_adapter: &R,
    _this_arg: &R::JsValueAdapterType,
    args: &[R::JsValueAdapterType],
) -> Result<R::JsValueAdapterType, JsError> {
    if log::max_level() >= LevelFilter::Error {
        log::error!("{}", parse_line(realm_adapter, args));
    }

    realm_adapter.js_null_create()
}

fn console_warn<R: JsRealmAdapter>(
    _runtime_adapter: &R::JsRuntimeAdapterType,
    realm_adapter: &R,
    _this_arg: &R::JsValueAdapterType,
    args: &[R::JsValueAdapterType],
) -> Result<R::JsValueAdapterType, JsError> {
    if log::max_level() >= LevelFilter::Warn {
        log::warn!("{}", parse_line(realm_adapter, args));
    }

    realm_adapter.js_null_create()
}

fn console_debug<R: JsRealmAdapter>(
    _runtime_adapter: &R::JsRuntimeAdapterType,
    realm_adapter: &R,
    _this_arg: &R::JsValueAdapterType,
    args: &[R::JsValueAdapterType],
) -> Result<R::JsValueAdapterType, JsError> {
    if log::max_level() >= LevelFilter::Debug {
        log::debug!("{}", parse_line(realm_adapter, args));
    }

    realm_adapter.js_null_create()
}

fn console_trace<R: JsRealmAdapter>(
    _runtime_adapter: &R::JsRuntimeAdapterType,
    realm_adapter: &R,
    _this_arg: &R::JsValueAdapterType,
    args: &[R::JsValueAdapterType],
) -> Result<R::JsValueAdapterType, JsError> {
    if log::max_level() >= LevelFilter::Trace {
        log::trace!("{}", parse_line(realm_adapter, args));
    }

    realm_adapter.js_null_create()
}

fn parse_field_value<R: JsRealmAdapter>(
    realm_adapter: &R,
    field: &str,
    value: &R::JsValueAdapterType,
) -> String {
    // format ints
    // only support ,2 / .3 to declare the number of digits to display, e.g. $.3i turns 3 to 003

    // format floats
    // only support ,2 / .3 to declare the number of decimals to display, e.g. $.3f turns 3.1 to 3.100

    if field.eq(&"%.0f".to_string()) {
        return parse_field_value(realm_adapter, "%i", value);
    }

    if field.ends_with('d') || field.ends_with('i') {
        #[allow(clippy::or_fun_call)]
        let mut i_val: String = value.js_to_string().unwrap_or("".to_string());

        // remove chars behind .
        if let Some(i) = i_val.find('.') {
            let _ = i_val.split_off(i);
        }

        if let Some(dot_in_field_idx) = field.find('.') {
            let mut m_field = field.to_string();
            // get part behind dot
            let mut num_decimals_str = m_field.split_off(dot_in_field_idx + 1);
            // remove d or i at end
            let _ = num_decimals_str.split_off(num_decimals_str.len() - 1);
            // see if we have a number
            if !num_decimals_str.is_empty() {
                let ct_res = usize::from_str(num_decimals_str.as_str());
                // check if we can parse the number to a usize
                if let Ok(ct) = ct_res {
                    // and if so, make i_val longer
                    while i_val.len() < ct {
                        i_val = format!("0{}", i_val);
                    }
                }
            }
        }

        return i_val;
    } else if field.ends_with('f') {
        #[allow(clippy::or_fun_call)]
        let mut f_val: String = value.js_to_string().unwrap_or("".to_string());

        if let Some(dot_in_field_idx) = field.find('.') {
            let mut m_field = field.to_string();
            // get part behind dot
            let mut num_decimals_str = m_field.split_off(dot_in_field_idx + 1);
            // remove d or i at end
            let _ = num_decimals_str.split_off(num_decimals_str.len() - 1);
            // see if we have a number
            if !num_decimals_str.is_empty() {
                let ct_res = usize::from_str(num_decimals_str.as_str());
                // check if we can parse the number to a usize
                if let Ok(ct) = ct_res {
                    // and if so, make i_val longer
                    if ct > 0 {
                        if !f_val.contains('.') {
                            f_val.push('.');
                        }

                        let dot_idx = f_val.find('.').unwrap();

                        while f_val.len() - dot_idx <= ct {
                            f_val.push('0');
                        }
                        if f_val.len() - dot_idx > ct {
                            let _ = f_val.split_off(dot_idx + ct + 1);
                        }
                    }
                }
            }
            return f_val;
        } else if field.ends_with('o') || field.ends_with('O') {
            let json_str_res = realm_adapter.js_json_stringify(value, None);
            let json = match json_str_res {
                Ok(json_str) => json_str,
                Err(_e) => "".to_string(),
            };
            return json;
        }
    }
    #[allow(clippy::or_fun_call)]
    match value.js_get_type() {
        JsValueType::Object => stringify_log_obj(realm_adapter, value),
        JsValueType::Function => stringify_log_obj(realm_adapter, value),
        JsValueType::Array => stringify_log_obj(realm_adapter, value),
        _ => value.js_to_string().unwrap_or("".to_string()),
    }
}

fn parse_line<R: JsRealmAdapter>(realm_adapter: &R, args: &[R::JsValueAdapterType]) -> String {
    let mut output = String::new();

    output.push_str("JS_REALM:[");
    output.push_str(realm_adapter.js_get_realm_id());
    output.push_str("]: ");

    if args.is_empty() {
        return output;
    }

    let message = match &args[0].js_get_type() {
        JsValueType::Object => stringify_log_obj(realm_adapter, &args[0]),
        JsValueType::Function => stringify_log_obj(realm_adapter, &args[0]),
        JsValueType::Array => stringify_log_obj(realm_adapter, &args[0]),
        _ => args[0].js_to_string().unwrap_or_else(|e| format!("{}", e)),
    };

    let mut field_code = String::new();
    let mut in_field = false;

    let mut x = 1;

    let mut filled = 1;

    if args[0].js_is_string() {
        for chr in message.chars() {
            if in_field {
                field_code.push(chr);
                if chr.eq(&'s') || chr.eq(&'d') || chr.eq(&'f') || chr.eq(&'o') || chr.eq(&'i') {
                    // end field

                    if x < args.len() {
                        output.push_str(
                            parse_field_value(realm_adapter, field_code.as_str(), &args[x])
                                .as_str(),
                        );
                        x += 1;
                        filled += 1;
                    }

                    in_field = false;
                    field_code = String::new();
                }
            } else if chr.eq(&'%') {
                in_field = true;
            } else {
                output.push(chr);
            }
        }
    } else {
        output.push_str(message.as_str());
    }

    for arg in args.iter().skip(filled) {
        // add args which we're not filled in str
        output.push(' ');
        let tail_arg = match arg.js_get_type() {
            JsValueType::Object => stringify_log_obj(realm_adapter, arg),
            JsValueType::Function => stringify_log_obj(realm_adapter, arg),
            JsValueType::Array => stringify_log_obj(realm_adapter, arg),
            _ => arg.js_to_string().unwrap_or_else(|_| "".to_string()),
        };
        output.push_str(tail_arg.as_str());
    }

    output
}

#[cfg(test)]
pub mod tests {

    use hirofa_utils::js_utils::Script;
    use quickjs_runtime::builder::QuickJsRuntimeBuilder;
    //use log::LevelFilter;

    #[test]
    pub fn test_console() {
        //simple_logging::log_to_stderr(LevelFilter::Info);
        log::info!("> test_console");
        let rt = crate::init_greco_rt(QuickJsRuntimeBuilder::new()).build();

        rt.eval_sync(Script::new(
            "test_console.es",
            "console.log('one %s', 'two', 3);\
            console.log('two %s %s', 'two', 3);\
            console.log('date:', new Date());\
            console.log('err:', new Error('testpoof'));\
            console.log('array:', [1, 2, true, {a: 1}]);\
            console.log('obj arg: %s', {a: 1});\
            console.log({obj: true}, {obj: false});",
        ))
        .ok()
        .expect("test_console.es failed");
        log::info!("< test_console");
    }
}
