use hirofa_utils::js_utils::facades::JsRuntimeBuilder;

#[cfg(feature = "console")]
pub mod js_console;
#[cfg(feature = "fetch")]
pub mod js_fetch;
#[cfg(feature = "commonjs")]
pub mod require;

pub(crate) fn init<T: JsRuntimeBuilder>(builder: T) -> T {
    #[cfg(feature = "commonjs")]
    let builder = require::init(builder);

    #[cfg(feature = "http")]
    let builder = js_fetch::init(builder);

    js_console::init(builder)
}
