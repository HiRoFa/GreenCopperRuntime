use quickjs_runtime::builder::QuickJsRuntimeBuilder;

#[cfg(any(feature = "all", feature = "util", feature = "cache"))]
pub mod cache;

pub(crate) fn init(builder: QuickJsRuntimeBuilder) -> QuickJsRuntimeBuilder {
    // todo
    #[cfg(any(feature = "all", feature = "util", feature = "cache"))]
    let builder = cache::init(builder);

    builder
}
