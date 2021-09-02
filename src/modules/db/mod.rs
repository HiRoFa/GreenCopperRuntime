use hirofa_utils::js_utils::facades::JsRuntimeBuilder;

#[cfg(any(feature = "all", feature = "db", feature = "mysql"))]
pub mod mysql;

pub(crate) fn init<B: JsRuntimeBuilder>(builder: &mut B) {
    #[cfg(any(feature = "all", feature = "db", feature = "mysql"))]
    mysql::init(builder);
}
