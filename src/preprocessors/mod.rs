//! IfDefPreProcessor
//!
//! this is a script preprocessor which can be used to conditionally load/unload pieces of script before compilation or evaluation
//!
//! # Example
//! ```javascript
//!
//! ```

use crate::preprocessors::cpp::CppPreProcessor;
use crate::preprocessors::macros::MacrosPreProcessor;
use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;

pub mod cpp;
pub mod macros;

pub(crate) fn init(builder: EsRuntimeBuilder) -> EsRuntimeBuilder {
    builder
    //.script_pre_processor(MacrosPreProcessor::new())
    //.script_pre_processor(CppPreProcessor::new().default_extensions())
}
