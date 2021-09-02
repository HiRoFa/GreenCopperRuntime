//! IfDefPreProcessor
//!
//! this is a script preprocessor which can be used to conditionally load/unload pieces of script before compilation or evaluation
//!
//! # Example
//! ```javascript
//!
//! ```

use crate::preprocessors::cpp::CppPreProcessor;
use hirofa_utils::js_utils::facades::JsRuntimeBuilder;

pub mod cpp;
pub mod macros;

pub fn init<T: JsRuntimeBuilder>(builder: &mut T) {
    builder.js_script_pre_processor(CppPreProcessor::new());
}
