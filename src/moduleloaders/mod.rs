use log::trace;
use quickjs_runtime::quickjsruntime::ScriptModuleLoader;
use std::fs;
use std::path::Path;

pub struct FileSystemModuleLoader {
    base_path: &'static str,
}

impl FileSystemModuleLoader {
    pub fn new() -> Self {
        Self {
            base_path: "../scripts",
        }
    }

    fn read_file(&self, filename: &str) -> std::io::Result<String> {
        let path = format!("{}/{}", self.base_path, filename);

        trace!("FileSystemModuleLoader::read_file -> {}", &path);

        fs::read_to_string(path)
    }

    fn file_exists(&self, filename: &str) -> bool {
        let path = format!("{}/{}", self.base_path, filename);
        trace!("FileSystemModuleLoader::file_exists -> {}", &path);
        Path::new(path.as_str()).exists()
    }
}

impl ScriptModuleLoader for FileSystemModuleLoader {
    fn normalize_path(&self, ref_path: &str, path: &str) -> Option<String> {
        // the ref path will always be an absolute path, so no need to parse . or ..
        // but even though we call it an absolute path here it will will be a relative path to the loader's main dir
        // so basically the file:// prefix is just to recognize the path a a path the FileSystemModuleLoader can handle
        if ref_path.starts_with("file://") {
            // do my thing
        }
        None
    }

    fn load_module(&self, absolute_path: &str) -> String {
        unimplemented!()
    }
}

pub struct HttpModuleLoader {
    secure_only: bool,
    allowed_domains: Option<Vec<String>>,
    basic_auth: Option<(String, String)>,
    // todo stuff like clientcert / servercert checking
}

impl HttpModuleLoader {
    pub fn new() -> Self {
        Self {
            secure_only: false,
            allowed_domains: None,
            basic_auth: None,
        }
    }
}

impl ScriptModuleLoader for HttpModuleLoader {
    fn normalize_path(&self, ref_path: &str, path: &str) -> Option<String> {
        // the ref path will always be an absolute path

        if ref_path.starts_with("http://") || ref_path.starts_with("https://") {
            // do my thing
        }

        None
    }

    fn load_module(&self, absolute_path: &str) -> String {
        unimplemented!()
    }
}
