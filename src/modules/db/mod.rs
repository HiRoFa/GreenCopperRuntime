use quickjs_runtime::builder::QuickJsRuntimeBuilder;

#[cfg(any(feature = "all", feature = "db", feature = "mysql"))]
pub mod mysql;

#[cfg(any(feature = "all", feature = "db", feature = "sqlx"))]
pub mod sqlx;

pub(crate) fn init(builder: QuickJsRuntimeBuilder) -> QuickJsRuntimeBuilder {
    #[cfg(any(feature = "all", feature = "db", feature = "mysql"))]
    let builder = mysql::init(builder);
    #[cfg(any(feature = "all", feature = "db", feature = "sqlx"))]
    let builder = sqlx::init(builder);
    builder
}
