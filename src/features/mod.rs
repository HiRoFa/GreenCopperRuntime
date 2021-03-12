use quickjs_runtime::eserror::EsError;
use quickjs_runtime::quickjscontext::QuickJsContext;
use quickjs_runtime::quickjsruntime::QuickJsRuntime;

pub(crate) fn init(_q_js_rt: &QuickJsRuntime, _q_ctx: &QuickJsContext) -> Result<(), EsError> {
    Ok(())
}
