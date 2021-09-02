/*

revisit this when figured out how to add custom client to fetch api like Deno does

#[cfg(any(feature = "all", feature = "com", feature = "http"))]
pub mod http;

pub(crate) fn init<B: JsRuntimeBuilder>(builder: &mut B) {
    #[cfg(any(feature = "all", feature = "com", feature = "http"))]
    http::init(builder);
}

 */
