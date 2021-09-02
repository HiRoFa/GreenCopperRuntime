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
//! use quickjs_runtime::esvalue::JsValueFacade;
//! use hirofa_utils::js_utils::Script;
//! let rt = crate::green_copper_runtime::new_greco_rt_builder().build();
//! rt.eval_sync(Script::new("init_fs.es", "async function test_write() {\
//!     let fs_mod = await import('greco://fs');\
//!     await fs_mod.write('./test.txt', 'hello from greco fs');
//! }\n"));
//! let prom_esvf = rt.call_function_sync(vec![], "test_write", vec![]).ok().expect("write function invocation failed");
//! // wait for promise to be done
//! let done = prom_esvf.get_promise_result_sync();
//! assert!(done.is_ok());
//! // do read test
//! rt.eval_sync(Script::new("init_fs.es", "async function test_read() {\
//!     let fs_mod = await import('greco://fs');\
//!     return await fs_mod.readString('./test.txt');
//! }\n"));
//! let prom_esvf = rt.call_function_sync(vec![], "test_read", vec![]).ok().expect("read invocation failed");
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
//! ##append
//! ##copy
//! ##createSymlink
//! ##createDirs
//! ##getMetadata
//! ##getSymlinkMetadata
//! ##list
//! ##readString
//! ##removeDir
//! ##removeFile
//! ##rename
//! ##touch
//! ##write
//!

use hirofa_utils::js_utils::adapters::JsRealmAdapter;
use hirofa_utils::js_utils::facades::values::JsValueFacade;
use hirofa_utils::js_utils::facades::JsRuntimeBuilder;
use hirofa_utils::js_utils::modules::NativeModuleLoader;
use hirofa_utils::js_utils::JsError;
use std::fs;

pub(crate) fn read_string(args: &[JsValueFacade]) -> Result<JsValueFacade, JsError> {
    if args.len() != 1 || !args[0].is_string() {
        Err(JsError::new_str(
            "readString requires one argument: (String)",
        ))
    } else {
        let path = args[0].get_str();

        match fs::read_to_string(path) {
            Ok(s) => Ok(JsValueFacade::new_string(s)),
            Err(e) => Err(JsError::new_string(format!("{}", e))),
        }
    }
}

pub(crate) fn remove_file(args: &[JsValueFacade]) -> Result<JsValueFacade, JsError> {
    if args.len() != 1 || !args[0].is_string() {
        Err(JsError::new_str(
            "removeFile requires one argument: (String)",
        ))
    } else {
        let path = args[0].get_str();

        match fs::remove_file(path) {
            Ok(_) => Ok(JsValueFacade::Null),
            Err(e) => Err(JsError::new_string(format!("{}", e))),
        }
    }
}

pub(crate) fn append(_args: &[JsValueFacade]) -> Result<JsValueFacade, JsError> {
    unimplemented!()
}

pub(crate) fn copy(_args: &[JsValueFacade]) -> Result<JsValueFacade, JsError> {
    unimplemented!()
}

pub(crate) fn create_symlink(_args: &[JsValueFacade]) -> Result<JsValueFacade, JsError> {
    unimplemented!()
}

pub(crate) fn create_dirs(_args: &[JsValueFacade]) -> Result<JsValueFacade, JsError> {
    unimplemented!()
}

pub(crate) fn get_metadata(_args: &[JsValueFacade]) -> Result<JsValueFacade, JsError> {
    unimplemented!()
}

pub(crate) fn get_symlink_metadata(_args: &[JsValueFacade]) -> Result<JsValueFacade, JsError> {
    unimplemented!()
}

pub(crate) fn list(_args: &[JsValueFacade]) -> Result<JsValueFacade, JsError> {
    unimplemented!()
}

pub(crate) fn remove_dir(_args: &[JsValueFacade]) -> Result<JsValueFacade, JsError> {
    unimplemented!()
}

pub(crate) fn rename(_args: &[JsValueFacade]) -> Result<JsValueFacade, JsError> {
    unimplemented!()
}

pub(crate) fn touch(_args: &[JsValueFacade]) -> Result<JsValueFacade, JsError> {
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
pub(crate) fn write(args: &[JsValueFacade]) -> Result<JsValueFacade, JsError> {
    if args.len() != 2 || !args[0].is_string() {
        Err(JsError::new_str(
            "write requires two arguments: (String, obj)",
        ))
    } else {
        let path = args[0].get_str();
        let content = if args[1].is_string() {
            args[1].get_str().to_string()
        } else {
            args[1].stringify()
        };

        match fs::write(path, content) {
            Ok(_) => Ok(JsValueFacade::Null),
            Err(e) => Err(JsError::new_string(format!("{}", e))),
        }
    }
}

pub struct FsModuleLoader {}

impl<R: JsRealmAdapter + 'static> NativeModuleLoader<R> for FsModuleLoader {
    fn has_module(&self, _realm: &R, module_name: &str) -> bool {
        module_name.eq("greco://fs")
    }

    fn get_module_export_names(&self, _realm: &R, _module_name: &str) -> Vec<&str> {
        vec![
            "append",
            "copy",
            "createSymlink",
            "createDirs",
            "getMetadata",
            "getSymlinkMetadata",
            "list",
            "readString",
            "removeDir",
            "removeFile",
            "rename",
            "touch",
            "write",
        ]
    }

    fn get_module_exports(
        &self,
        realm: &R,
        _module_name: &str,
    ) -> Vec<(&str, R::JsValueAdapterType)> {
        init_exports(realm).ok().expect("init fs exports failed")
    }
}

pub(crate) fn init<B: JsRuntimeBuilder>(builder: &mut B) {
    builder.js_native_module_loader(FsModuleLoader {});
}

fn init_exports<R: JsRealmAdapter + 'static>(
    realm: &R,
) -> Result<Vec<(&'static str, R::JsValueAdapterType)>, JsError> {
    let copy_func = JsValueFacade::new_function("copy", copy, 1);
    let write_func = JsValueFacade::new_function("write", write, 1);
    let append_func = JsValueFacade::new_function("append", append, 1);
    let create_symlink_func = JsValueFacade::new_function("createSymlink", create_symlink, 1);
    let create_dirs_func = JsValueFacade::new_function("createDirs", create_dirs, 1);
    let get_metadata_func = JsValueFacade::new_function("getMetadata", get_metadata, 1);
    let get_symlink_metadata_func =
        JsValueFacade::new_function("getSymlinkMetadata", get_symlink_metadata, 1);
    let list_func = JsValueFacade::new_function("list", list, 1);
    let remove_dir_func = JsValueFacade::new_function("removeDir", remove_dir, 1);
    let rename_func = JsValueFacade::new_function("rename", rename, 1);
    let touch_func = JsValueFacade::new_function("touch", touch, 1);
    let remove_file_func = JsValueFacade::new_function("removeFile", remove_file, 1);
    let read_string_func = JsValueFacade::new_function("readString", read_string, 1);

    Ok(vec![
        ("write", realm.from_js_value_facade(write_func)?),
        (
            "getSymlinkMetadata",
            realm.from_js_value_facade(get_symlink_metadata_func)?,
        ),
        ("copy", realm.from_js_value_facade(copy_func)?),
        ("append", realm.from_js_value_facade(append_func)?),
        (
            "createSymlink",
            realm.from_js_value_facade(create_symlink_func)?,
        ),
        ("createDirs", realm.from_js_value_facade(create_dirs_func)?),
        (
            "getMetadata",
            realm.from_js_value_facade(get_metadata_func)?,
        ),
        ("list", realm.from_js_value_facade(list_func)?),
        ("removeDir", realm.from_js_value_facade(remove_dir_func)?),
        ("rename", realm.from_js_value_facade(rename_func)?),
        ("touch", realm.from_js_value_facade(touch_func)?),
        ("removeFile", realm.from_js_value_facade(remove_file_func)?),
        ("readString", realm.from_js_value_facade(read_string_func)?),
    ])
}

#[cfg(test)]
pub mod tests {
    use crate::init_greco_rt;
    use backtrace::Backtrace;
    use hirofa_utils::js_utils::Script;
    use log::LevelFilter;
    use quickjs_runtime::builder::QuickJsRuntimeBuilder;
    use std::panic;

    #[test]
    fn test_fs() {
        panic::set_hook(Box::new(|panic_info| {
            let backtrace = Backtrace::new();
            log::error!(
                "thread panic occurred: {}\nbacktrace: {:?}",
                panic_info,
                backtrace
            );
        }));

        simple_logging::log_to_file("grecort.log", LevelFilter::max())
            .ok()
            .expect("could not init logger");

        let mut rtb = QuickJsRuntimeBuilder::new();
        init_greco_rt(&mut rtb);
        let rt = rtb.build();
        rt.eval_sync(Script::new(
            "init_fs.es",
            "async function test_write() {\
     let fs_mod = await import('greco://fs');\
     await fs_mod.write('./test.txt', 'hello from greco fs');
 }\n",
        ))
        .ok()
        .expect("init write script failed");
        let prom_esvf = rt
            .call_function_sync(vec![], "test_write", vec![])
            .ok()
            .expect("write function invocation failed");
        // wait for promise to be done
        let done = prom_esvf.get_promise_result_sync();
        assert!(done.is_ok());
        // do read test
        rt.eval_sync(Script::new(
            "init_fs.es",
            "async function test_read() {\
     let fs_mod = await import('greco://fs');\
     return await fs_mod.readString('./test.txt');
 }\n",
        ))
        .ok()
        .expect("init write script failed");
        let prom_esvf = rt
            .call_function_sync(vec![], "test_read", vec![])
            .ok()
            .expect("read invocation failed");
        // wait for promise to be done
        let done = prom_esvf.get_promise_result_sync();
        assert!(done.is_ok());
        let done_esvf = done.ok().unwrap();
        let s = done_esvf.get_str();
        assert_eq!(s, "hello from greco fs");
    }
}
