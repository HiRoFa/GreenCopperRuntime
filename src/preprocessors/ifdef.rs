use quickjs_runtime::esruntime::ScriptPreProcessor;
use quickjs_runtime::esscript::EsScript;

pub struct IfDefPreProcessor {
    defs: Vec<&'static str>,
    extensions: Vec<&'static str>,
}

impl IfDefPreProcessor {
    pub fn new() -> Self {
        Self {
            defs: vec![],
            extensions: vec![],
        }
    }
    /// add a def
    pub fn def(mut self, var: &'static str) -> Self {
        self.defs.push(var);
        self
    }
    /// add a supported extension e.g. js/mjs/ts/mts/es/mes
    pub fn extension(mut self, ext: &'static str) -> Self {
        self.extensions.push(ext);
        self
    }
    /// add default extensions : js/mjs/ts/mts/es/mes
    pub fn default_extensions(self) -> Self {
        self.extension("es")
            .extension("mes")
            .extension("js")
            .extension("mjs")
            .extension("ts")
            .extension("mts")
    }
}

impl ScriptPreProcessor for IfDefPreProcessor {
    fn process(&self, script: EsScript) -> EsScript {
        script
    }
}
