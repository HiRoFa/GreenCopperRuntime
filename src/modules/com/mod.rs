use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;

#[cfg(any(feature = "all", feature = "com", feature = "http"))]
pub mod http;

pub(crate) fn init(builder: EsRuntimeBuilder) -> EsRuntimeBuilder {
    #[cfg(any(feature = "all", feature = "com", feature = "http"))]
    let builder = http::init(builder);

    builder
}
