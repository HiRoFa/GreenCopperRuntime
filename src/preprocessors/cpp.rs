//! cpp style preprocessor
//!
//! this can be used to define c-like preprocessing instructions in javascript
//! ```javascript
//!     function do_stuff(input) {
//!         #ifdef $GRECO_DEBUG
//!         if (input.includes('booh')) {
//!             throw Error('input should not include booh');
//!         }
//!         #endif
//!         console.log('got input %s', input);
//!     }
//! ```
//!
//! it used the gpp crate and docs on how it works are here https://docs.rs/gpp/0.6.0/gpp
//!
//! by default GreenCopperRuntime conditionally sets the $GRECO_DEBUG, $GRECO_TEST and $GRECO_RELEASE
//! you can also add all current env_vars so in script you can use ```let path = "$PATH";```;
//!
//! # Example
//! ```rust
//! use green_copper_runtime::preprocessors::cpp::CppPreProcessor;
//! use hirofa_utils::js_utils::Script;
//! use quickjs_runtime::builder::QuickJsRuntimeBuilder;
//!
//! let cpp = CppPreProcessor::new().default_extensions().env_vars();
//! let rt = QuickJsRuntimeBuilder::new().script_pre_processor(cpp).build();
//!
//! let path = rt.eval_sync(Script::new("test.js", "let p = '$PATH'; p")).ok().expect("script failed");
//! assert!(!path.get_str().is_empty());
//! assert_ne!(path.get_str(), "$PATH");
//!
//! ```
//!

use gpp::{process_str, Context};
use hirofa_utils::js_utils::{JsError, Script, ScriptPreProcessor};
use std::cell::RefCell;
use std::env;

pub struct CppPreProcessor {
    ctx: RefCell<Context>,
    extensions: Vec<&'static str>,
}

impl Default for CppPreProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl CppPreProcessor {
    pub fn new() -> Self {
        let mut ret = Self {
            ctx: RefCell::new(Context::new()),
            extensions: vec![],
        };

        #[cfg(debug_assertions)]
        {
            ret = ret.def("GRECO_DEBUG", "true");
        }
        #[cfg(test)]
        {
            ret = ret.def("GRECO_TEST", "true");
        }
        #[cfg(not(any(debug_assertions, test)))]
        {
            ret = ret.def("GRECO_RELEASE", "true");
        }

        ret
    }
    /// add a def
    pub fn def(self, key: &str, value: &str) -> Self {
        {
            let ctx = &mut *self.ctx.borrow_mut();

            ctx.macros
                .insert(format!("${{{}}}", key), value.to_string());
            ctx.macros.insert(format!("${}", key), value.to_string());
            ctx.macros.insert(format!("__{}", key), value.to_string());
        }
        self
    }
    /// add a supported extension e.g. js/mjs/ts/mts/es/mes
    pub fn extension(mut self, ext: &'static str) -> Self {
        self.extensions.push(ext);
        self
    }

    pub fn env_vars(mut self) -> Self {
        log::debug!("adding env vars");
        for (key, value) in env::vars() {
            log::debug!("adding env var {} = {}", key, value);
            self = self.def(key.as_str(), value.as_str());
        }
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
    fn process(&self, script: &mut Script) -> Result<(), JsError> {
        if "CppPreProcessor.not_es".eq(script.get_path()) {
            return Ok(());
        }

        log::debug!("CppPreProcessor > {}", script.get_path());

        let src = script.get_code();

        let res = process_str(src, &mut self.ctx.borrow_mut())
            .map_err(|e| JsError::new_string(format!("{}", e)))?;

        script.set_code(res);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::init_test_greco_rt;
    use futures::executor::block_on;
    use hirofa_utils::js_utils::facades::values::JsValueFacade;
    use hirofa_utils::js_utils::facades::JsRuntimeFacade;
    use hirofa_utils::js_utils::Script;

    #[test]
    fn test_ifdef() {
        let rt = init_test_greco_rt();
        let fut = rt.js_eval(
            None,
            Script::new(
                "test.es",
                "((function(){\n\
        #ifdef HELLO\n\
            return 111;\n\
        #elifdef $GRECO_DEBUG\n\
            return 123;\n\
        #else\n\
            return 222;\n\
        #endif\n\
        })());",
            ),
        );
        let res = block_on(fut);
        let num = match res {
            Ok(e) => e,
            Err(err) => {
                panic!("{}", err);
            }
        };
        if let JsValueFacade::I32 { val } = num {
            assert_eq!(val, 123);
        } else {
            panic!("not an i32")
        }
    }

    #[test]
    fn test_vars() {
        let rt = init_test_greco_rt();
        let fut = rt.js_eval(
            None,
            Script::new(
                "test.es",
                "((function(){\n\
        return('p=${PATH}');\n\
        })());",
            ),
        );
        let res = block_on(fut);
        let val = match res {
            Ok(e) => e,
            Err(err) => {
                panic!("{}", err);
            }
        };

        if let JsValueFacade::String { val } = val {
            assert_ne!(&*val, "${PATH}");
            assert!(!val.is_empty());
        } else {
            panic!("not a string")
        }
    }
}
