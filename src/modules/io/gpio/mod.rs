use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;
use quickjs_runtime::quickjscontext::QuickJsContext;
use quickjs_runtime::quickjsruntime::NativeModuleLoader;
use quickjs_runtime::valueref::JSValueRef;

struct GpioModuleLoader {}

impl NativeModuleLoader for GpioModuleLoader {
    fn has_module(&self, _q_ctx: &QuickJsContext, module_name: &str) -> bool {
        module_name.eq("greco://gpio")
    }

    fn get_module_export_names(&self, _q_ctx: &QuickJsContext, _module_name: &str) -> Vec<&str> {
        vec!["PinSet"]
    }

    fn get_module_exports(
        &self,
        _q_ctx: &QuickJsContext,
        _module_name: &str,
    ) -> Vec<(&str, JSValueRef)> {
        unimplemented!()
    }
}

pub(crate) fn init(builder: EsRuntimeBuilder) -> EsRuntimeBuilder {
    builder.native_module_loader(Box::new(GpioModuleLoader {}))
}
