use crate::new_greco_rt_builder2;
use quickjs_runtime::esruntime::{EsRuntime, ScriptPreProcessor};
use quickjs_runtime::esscript::EsScript;
use quickjs_runtime::esvalue::EsValueConvertible;
use quickjs_runtime::quickjs_utils::get_global_q;
use quickjs_runtime::quickjs_utils::objects::get_property_q;
use std::sync::Arc;

lazy_static! {
    static ref UTIL_RT: Arc<EsRuntime> = new_greco_rt_builder2(false, true, true).build();
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
    fn process(&self, mut script: EsScript) -> EsScript {
        if "CppPreProcessor.not_es".eq(script.get_path()) {
            return script;
        }

        log::debug!("CppPreProcessor > {}", script.get_path());

        let rt: Arc<EsRuntime> = UTIL_RT.clone();

        rt.exe_rt_task_in_event_loop(|q_js_rt| {
            // todo create specific context?
            let q_ctx = q_js_rt.get_main_context();
            let global = get_global_q(q_ctx);
            let obj = get_property_q(q_ctx, &global, "CppPreProcessor")
                .ok()
                .expect("get CppPreProcessor failed");
            if obj.is_null_or_undefined() {
                // init func
                q_ctx
                    .eval(EsScript::new(
                        "CppPreProcessor.not_es",
                        "this.CppPreProcessor = {process: function(src, ...vars){\
                        let compiler = require('https://raw.githubusercontent.com/ParksProjets/C-Preprocessor/master/lib/compiler.js');\
                        let options = {constants: {DEBUG: true}}; /* todo replace with vars */\
                        return new Promise((resolve, reject) => {\
                             compiler.compile(src, options, (err, result) => {\
                                 if result {\
                                    resolve(result);\
                                 } else {\
                                    reject(err);\
                                 }\
                             });\
                        });\
                        
                    }};",
                    ))
                    .ok()
                    .expect("CppPreProcessor init script failed");
            }


        });

        let src = script.get_code().to_string().to_es_value_facade();
        let proc_res_prom = rt
            .call_function_sync(vec!["CppPreProcessor"], "process", vec![src])
            .ok()
            .expect("proc func failed");

        let proc_res = proc_res_prom.get_promise_result_sync();
        let new_code = proc_res.ok().expect("prom did not resolve");
        script.set_code(new_code.get_str().to_string());
        script
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::init_test_greco_rt;
    use quickjs_runtime::esscript::EsScript;

    #[test]
    fn test_ifdef() {
        let rt = init_test_greco_rt();
        let res = rt
            .eval_sync(EsScript::new("test.es", "(123);"))
            .ok()
            .expect("script failed");
        assert_eq!(res.get_i32(), 123);
    }
}
