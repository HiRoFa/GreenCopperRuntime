use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;

#[cfg(any(feature = "all", feature = "db", feature = "mysql"))]
pub mod mysql;

pub(crate) fn init(builder: EsRuntimeBuilder) -> EsRuntimeBuilder {
    #[cfg(any(feature = "all", feature = "db", feature = "mysql"))]
    let builder = mysql::init(builder);
    builder
}
