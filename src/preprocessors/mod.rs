//! IfDefPreProcessor
//!
//! this is a script preprocessor which can be used to conditionally load/unload pieces of script before compilation or evaluation
//!
//! # Example
//! ```javascript
//!
//! ```

use crate::preprocessors::cpp::CppPreProcessor;
use quickjs_runtime::builder::QuickJsRuntimeBuilder;

pub mod cpp;
pub mod macros;

pub fn init(builder: QuickJsRuntimeBuilder) -> QuickJsRuntimeBuilder {
    builder.script_pre_processor(CppPreProcessor::new())
}
