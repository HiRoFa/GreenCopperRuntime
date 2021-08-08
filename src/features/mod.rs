#[cfg(feature = "quickjs")]
use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;

#[cfg(feature = "console")]
pub mod js_console;
#[cfg(feature = "fetch")]
pub mod js_fetch;
#[cfg(feature = "commonjs")]
pub mod require;

#[cfg(feature = "quickjs")]
pub(crate) fn init(builder: EsRuntimeBuilder) -> EsRuntimeBuilder {
    #[cfg(feature = "commonjs")]
    let mut builder = require::init(builder);

    js_fetch::init(&mut builder);
    js_console::init(&mut builder);

    builder
}
