use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;

#[cfg(feature = "require")]
pub mod require;

pub(crate) fn init(builder: EsRuntimeBuilder) -> EsRuntimeBuilder {
    #[cfg(feature = "require")]
    let builder = require::init(builder);

    builder
}
