//! #FS Module
//!
//! ```javascript
//! async function test() {
//!     let fs_mod = await import('greco://fs');
//!     await fs_mod.write('./test.txt', 'hello from greco fs');
//! }
//! ```
//! # example
//!
//! ```rust
//! use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;
//! use quickjs_runtime::esscript::EsScript;
//! let rt = crate::green_copper_runtime::new_greco_rt_builder().build();
//! let prom_esvf = rt.eval_sync(EsScript::new("test_fs.es", "\
//! (async function test() {\
//!     let fs_mod = await import('greco://fs');\
//!     await fs_mod.write('./test.txt', 'hello from greco fs');\
//! }\
//! test())\
//! ")).ok().expect("write script failed");
//! // wait for promise to be done
//! let done = prom_esvf.get_promise_result_sync();
//! assert!(done.is_ok());
//! // do read test
//! let prom_esvf = rt.eval_sync(EsScript::new("test_fs_read.es", "\
//! (async function test_read() {\
//!     let fs_mod = await import('greco://fs');\
//!     return await fs_mod.readString('./test.txt');\
//! }\
//! test_read())\
//! ")).ok().expect("read script failed");
//! // wait for promise to be done
//! let done = prom_esvf.get_promise_result_sync();
//! assert!(done.is_ok());
//! let done_esvf = done.ok().unwrap();
//! let s = done_esvf.get_str();
//! assert_eq!(s, "hello from greco fs");
//! ```
//!
//! # Methods
//!
//! ## readString
//!
//! ## write
//!
//! ## delete
//!
//! ## touch
//!

use quickjs_runtime::eserror::EsError;
use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;
use quickjs_runtime::esvalue::{EsFunction, EsValueConvertible, EsValueFacade};
use quickjs_runtime::quickjscontext::QuickJsContext;
use quickjs_runtime::quickjsruntime::NativeModuleLoader;
use quickjs_runtime::valueref::JSValueRef;

pub(crate) fn read_string(_args: Vec<EsValueFacade>) -> Result<EsValueFacade, String> {
    unimplemented!()
}

pub(crate) fn delete(_args: Vec<EsValueFacade>) -> Result<EsValueFacade, String> {
    unimplemented!()
}

pub(crate) fn touch(_args: Vec<EsValueFacade>) -> Result<EsValueFacade, String> {
    unimplemented!()
}

/// write
/// write to a file
/// # Example
/// ```javascript
/// async function write_example() {
///    let fs = await import('greco://fs');
///    await fs.write('./test.txt', 'hello world');
/// }
/// ```
pub(crate) fn write(_args: Vec<EsValueFacade>) -> Result<EsValueFacade, String> {
    unimplemented!()
}

pub struct FsModuleLoader {}

impl NativeModuleLoader for FsModuleLoader {
    fn has_module(&self, _q_ctx: &QuickJsContext, module_name: &str) -> bool {
        module_name.eq("greco://fs")
    }

    fn get_module_export_names(&self, _q_ctx: &QuickJsContext, _module_name: &str) -> Vec<&str> {
        vec!["readString", "write", "delete", "touch"]
    }

    fn get_module_exports(
        &self,
        q_ctx: &QuickJsContext,
        _module_name: &str,
    ) -> Vec<(&str, JSValueRef)> {
        init_exports(q_ctx).ok().expect("init fs exports failed")
    }
}

pub(crate) fn init(builder: EsRuntimeBuilder) -> EsRuntimeBuilder {
    builder.native_module_loader(Box::new(FsModuleLoader {}))
}

fn init_exports(q_ctx: &QuickJsContext) -> Result<Vec<(&'static str, JSValueRef)>, EsError> {
    let write_func = EsFunction::new("write", write, true);
    let touch_func = EsFunction::new("touch", touch, true);
    let delete_func = EsFunction::new("delete", delete, true);
    let read_string_func = EsFunction::new("readString", read_string, true);

    Ok(vec![
        ("write", write_func.to_es_value_facade().as_js_value(q_ctx)?),
        ("touch", touch_func.to_es_value_facade().as_js_value(q_ctx)?),
        (
            "delete",
            delete_func.to_es_value_facade().as_js_value(q_ctx)?,
        ),
        (
            "readString",
            read_string_func.to_es_value_facade().as_js_value(q_ctx)?,
        ),
    ])
}
