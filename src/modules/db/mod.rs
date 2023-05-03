use quickjs_runtime::builder::QuickJsRuntimeBuilder;

#[cfg(any(feature = "all", feature = "db", feature = "mysql"))]
pub mod mysql;

pub(crate) fn init(builder: QuickJsRuntimeBuilder) -> QuickJsRuntimeBuilder {
    #[cfg(any(feature = "all", feature = "db", feature = "mysql"))]
    mysql::init(builder)
}
