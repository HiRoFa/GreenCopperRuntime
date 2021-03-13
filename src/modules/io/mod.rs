use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;

#[cfg(any(feature = "all", feature = "io", feature = "gpio"))]
mod gpio;

pub(crate) fn init(builder: EsRuntimeBuilder) -> EsRuntimeBuilder {
    #[cfg(any(feature = "all", feature = "io", feature = "gpio"))]
    let builder = gpio::init(builder);

    builder
}
