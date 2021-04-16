use quickjs_runtime::eserror::EsError;
use quickjs_runtime::esruntime::ScriptPreProcessor;
use quickjs_runtime::esscript::EsScript;

pub struct MacrosPreProcessor {}

impl MacrosPreProcessor {
    pub fn new() -> Self {
        Self {}
    }
}

impl ScriptPreProcessor for MacrosPreProcessor {
    fn process(&self, script: EsScript) -> Result<EsScript, EsError> {
        Ok(script)
    }
}

impl Default for MacrosPreProcessor {
    fn default() -> Self {
        Self::new()
    }
}
