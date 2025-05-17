use quickjs_runtime::builder::QuickJsRuntimeBuilder;

#[cfg(feature = "sqlx")]
pub mod sqlx;

pub(crate) fn init(builder: QuickJsRuntimeBuilder) -> QuickJsRuntimeBuilder {
    #[cfg(feature = "sqlx")]
    let builder = sqlx::init(builder);
    builder
}
