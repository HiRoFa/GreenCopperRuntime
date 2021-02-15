use quickjs_runtime::esruntime::EsRuntime;
use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;
use std::sync::Arc;

pub fn new_runtime(builder: EsRuntimeBuilder) -> Arc<EsRuntime> {
    let rt = builder.build();

    // add init code to rt here

    rt
}
