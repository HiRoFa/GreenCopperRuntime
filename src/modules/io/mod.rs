use hirofa_utils::js_utils::facades::JsRuntimeBuilder;

#[cfg(any(feature = "all", feature = "io", feature = "gpio"))]
pub mod gpio;

#[cfg(any(feature = "all", feature = "io", feature = "fs"))]
pub mod fs;

pub(crate) fn init<B: JsRuntimeBuilder>(builder: &mut B) {
    #[cfg(any(feature = "all", feature = "io", feature = "gpio"))]
    gpio::init(builder);

    #[cfg(any(feature = "all", feature = "io", feature = "fs"))]
    fs::init(builder);
}
