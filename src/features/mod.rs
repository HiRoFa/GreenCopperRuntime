use hirofa_utils::js_utils::facades::JsRuntimeBuilder;

#[cfg(feature = "console")]
pub mod js_console;
#[cfg(feature = "fetch")]
pub mod js_fetch;
#[cfg(feature = "commonjs")]
pub mod require;

pub(crate) fn init<T: JsRuntimeBuilder>(builder: &mut T) {
    #[cfg(feature = "commonjs")]
    require::init(builder);

    #[cfg(feature = "http")]
    js_fetch::init(builder);

    js_console::init(builder);
}
