use hirofa_utils::js_utils::facades::JsRuntimeBuilder;

#[cfg(any(feature = "all", feature = "utils", feature = "cache"))]
pub mod cache;

pub(crate) fn init<B: JsRuntimeBuilder>(builder: B) -> B {
    // todo
    #[cfg(any(feature = "all", feature = "utils", feature = "cache"))]
    let builder = cache::init(builder);

    builder
}
