use quickjs_runtime::builder::QuickJsRuntimeBuilder;

#[cfg(feature = "fetch")]
pub mod js_fetch;
#[cfg(feature = "commonjs")]
pub mod require;

pub(crate) fn init(builder: QuickJsRuntimeBuilder) -> QuickJsRuntimeBuilder {
    #[cfg(feature = "commonjs")]
    let builder = require::init(builder);

    #[cfg(feature = "http")]
    let builder = js_fetch::init(builder);

    builder
}
