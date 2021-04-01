use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;

#[cfg(feature = "commonjs")]
pub mod require;

pub(crate) fn init(builder: EsRuntimeBuilder) -> EsRuntimeBuilder {
    #[cfg(feature = "commonjs")]
    let builder = require::init(builder);

    builder
}
