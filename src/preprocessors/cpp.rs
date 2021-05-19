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
//! by default GreenCopperRuntime conditionally sets the $GRECO_DEBUG, $GRECO_TEST and $GRECO_RELEASE vars
//!

use gpp::{process_str, Context};
use hirofa_utils::js_utils::{JsError, Script, ScriptPreProcessor};

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
    fn process(&self, script: &mut Script) -> Result<(), JsError> {
        if "CppPreProcessor.not_es".eq(script.get_path()) {
            return Ok(());
        }

        log::debug!("CppPreProcessor > {}", script.get_path());

        let src = script.get_code();

        let mut ctx = Context::new();

        #[cfg(debug_assertions)]
        {
            ctx.macros
                .insert("$GRECO_DEBUG".to_string(), "true".to_string());
        }
        #[cfg(test)]
        {
            ctx.macros
                .insert("$GRECO_TEST".to_string(), "true".to_string());
        }
        #[cfg(not(any(debug_assertions, test)))]
        {
            ctx.macros
                .insert("$GRECO_RELEASE".to_string(), "true".to_string());
        }

        let res = process_str(src, &mut ctx).map_err(|e| JsError::new_string(format!("{}", e)))?;

        script.set_code(res);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::init_test_greco_rt;
    use hirofa_utils::js_utils::Script;

    #[test]
    fn test_ifdef() {
        let rt = init_test_greco_rt();
        let res = rt.eval_sync(Script::new(
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
