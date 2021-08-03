use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;

pub mod js_console;
pub mod js_fetch;
#[cfg(feature = "commonjs")]
pub mod require;

pub(crate) fn init(builder: EsRuntimeBuilder) -> EsRuntimeBuilder {
    #[cfg(feature = "commonjs")]
    let builder = require::init(builder);
    let builder = js_fetch::init(builder);

    builder
}
