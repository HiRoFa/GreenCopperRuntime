use crate::moduleloaders::{FileSystemModuleLoader, HttpModuleLoader};
use crate::new_greco_rt_builder2;
use gpp::{process_str, Context};
use quickjs_runtime::eserror::EsError;
use quickjs_runtime::esruntime::{EsRuntime, ScriptPreProcessor};
use quickjs_runtime::esscript::EsScript;
use std::sync::Arc;

lazy_static! {
    static ref UTIL_RT: Arc<EsRuntime> = new_greco_rt_builder2(false, true, true)
        .script_module_loader(Box::new(
            HttpModuleLoader::new().validate_content_type(false)
        ))
        .script_module_loader(Box::new(FileSystemModuleLoader::new("./")))
        .build();
}

pub struct CppPreProcessor {
    defs: Vec<&'static str>,
    extensions: Vec<&'static str>,
}

impl Default for CppPreProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl CppPreProcessor {
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

impl ScriptPreProcessor for CppPreProcessor {
    fn process(&self, mut script: EsScript) -> Result<EsScript, EsError> {
        if "CppPreProcessor.not_es".eq(script.get_path()) {
            return Ok(script);
        }

        log::debug!("CppPreProcessor > {}", script.get_path());

        let src = script.get_code();

        let mut ctx = Context::new();

        #[cfg(debug_assertions)]
        {
            ctx.macros.insert("DEBUG".to_string(), "true".to_string());
        }
        #[cfg(test)]
        {
            ctx.macros.insert("TEST".to_string(), "true".to_string());
        }
        #[cfg(not(any(debug_assertions, test)))]
        {
            ctx.macros.insert("RELEASE".to_string(), "true".to_string());
        }

        let res = process_str(src, &mut ctx).map_err(|e| EsError::new_string(format!("{}", e)))?;

        script.set_code(res);
        Ok(script)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::init_test_greco_rt;
    use quickjs_runtime::esscript::EsScript;

    #[test]
    fn test_ifdef() {
        let rt = init_test_greco_rt();
        let res = rt.eval_sync(EsScript::new(
            "test.es",
            "((function(){\n\
        #ifdef HELLO\n\
            return 111;\n\
        #elifdef DEBUG\n\
            return 123;\n\
        #else\n\
            return 222;\n\
        #endif\n\
        })());",
        ));
        let num = match res {
            Ok(e) => e,
            Err(err) => {
                panic!("{}", err);
            }
        };
        assert_eq!(num.get_i32(), 123);
    }
}
