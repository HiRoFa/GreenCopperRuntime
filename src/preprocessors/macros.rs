use hirofa_utils::js_utils::{JsError, Script, ScriptPreProcessor};

pub struct MacrosPreProcessor {}

impl MacrosPreProcessor {
    pub fn new() -> Self {
        Self {}
    }
}

impl ScriptPreProcessor for MacrosPreProcessor {
    fn process(&self, _script: &mut Script) -> Result<(), JsError> {
        Ok(())
    }
}

impl Default for MacrosPreProcessor {
    fn default() -> Self {
        Self::new()
    }
}
