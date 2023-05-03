use quickjs_runtime::builder::QuickJsRuntimeBuilder;

#[cfg(any(feature = "all", feature = "io", feature = "gpio"))]
pub mod gpio;

#[cfg(any(feature = "all", feature = "io", feature = "fs"))]
pub mod fs;

pub(crate) fn init(builder: QuickJsRuntimeBuilder) -> QuickJsRuntimeBuilder {
    #[cfg(any(feature = "all", feature = "io", feature = "gpio"))]
    let builder = gpio::init(builder);

    #[cfg(any(feature = "all", feature = "io", feature = "fs"))]
    let builder = fs::init(builder);

    builder
}
