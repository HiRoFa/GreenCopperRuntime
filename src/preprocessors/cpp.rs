use crate::moduleloaders::{FileSystemModuleLoader, HttpModuleLoader};
use crate::new_greco_rt_builder2;
use quickjs_runtime::eserror::EsError;
use quickjs_runtime::esruntime::{EsRuntime, ScriptPreProcessor};
use quickjs_runtime::esscript::EsScript;
use quickjs_runtime::esvalue::EsValueConvertible;
use quickjs_runtime::quickjs_utils::get_global_q;
use quickjs_runtime::quickjs_utils::objects::get_property_q;
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

        let rt: Arc<EsRuntime> = UTIL_RT.clone();

        let init_res: Result<(), EsError> = rt.exe_rt_task_in_event_loop(|q_js_rt| {
            // todo create specific context?
            let q_ctx = q_js_rt.get_main_context();
            let global = get_global_q(q_ctx);
            let obj = get_property_q(q_ctx, &global, "CppPreProcessor")?;
            if obj.is_null_or_undefined() {
                // init func
                q_ctx
                    .eval(EsScript::new(
                        "CppPreProcessor.not_es",
                        "this.process = {};\n\
                            this.CppPreProcessor = {process: function(src, vars){\n\
                            let compiler_lib = require('https://raw.githubusercontent.com/ParksProjets/C-Preprocessor/master/lib/compiler.js');\n\
                            let compiler = new compiler_lib.Compiler();\n\
                            compiler.createConstant('DEBUG', 'true');\n\
                            let res = {};\n\
                            compiler.on('success', function(result) {\n\
                                res.result = result;\n\
                            });\n\
                            compiler.on('error', function(e) {\n\
                                res.err = e;\n\
                            });\n\
                            compiler.compile(src);\n\
                            if (res.result) {\n\
                                return(res.result);\n\
                            } else {\n\
                                throw Error(res.err);\n\
                            }\n\
                        }};",
                    ))?;
            }
            Ok(())

        });

        if init_res.is_err() {
            return Err(EsError::new_string(format!(
                "cpp preproc failed: {}",
                init_res.err().unwrap()
            )));
        }

        let src = script.get_code().to_string().to_es_value_facade();
        let new_code = rt.call_function_sync(vec!["CppPreProcessor"], "process", vec![src])?;
        script.set_code(new_code.get_str().to_string());
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
        #if 'DEBUG' == 'false'\n\
            return 111;\n\
        #elif 'DEBUG' == 'true'\n\
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
