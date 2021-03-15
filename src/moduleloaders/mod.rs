use log::trace;
use quickjs_runtime::eserror::EsError;
use quickjs_runtime::quickjsruntime::ScriptModuleLoader;
use std::fs;
use std::ops::Add;
use std::path::Path;
use url::Url;

pub struct FileSystemModuleLoader {
    base_path: &'static str,
}

fn last_index_of(haystack: &str, needle: &str) -> Option<usize> {
    let start = haystack.len() - needle.len();
    let mut x = start;
    loop {
        if haystack[x..(x + needle.len())].eq(needle) {
            return Some(x);
        }
        if x == 0 {
            break;
        }
        x = x - 1;
    }
    None
}

fn normalize_path(ref_path: &str, name: &str) -> Result<String, EsError> {
    // todo support:
    // name starting with /
    // name starting or containing ../ or starting with ./

    let ref_path = if let Some(last_slash_idx) = last_index_of(ref_path, "/") {
        let mut path = ref_path.to_string();
        let _file_name = path.split_off(last_slash_idx);
        path
    } else {
        ref_path.to_string()
    };

    let url = Url::parse(ref_path.as_str())
        .map_err(|e| EsError::new_string(format!("failed to parse Url: {}", e)))?;
    let path = if name.starts_with("/") {
        name.to_string()
    } else {
        format!("{}/{}", url.path(), name)
    };

    // remove ./
    // remove ..
    let mut path_parts: Vec<String> = path.split("/").into_iter().map(|s| s.to_string()).collect();

    let mut x = 1;
    while x < path_parts.len() {
        if path_parts[x].as_str().eq("..") {
            path_parts.remove(x);
            path_parts.remove(x - 1);
            x = 0;
        }
        if path_parts[x].as_str().eq(".") {
            path_parts.remove(x);
            x = 0;
        }
        x += 1;
    }
    let path = path_parts.join("/");

    let mut res = url.scheme().to_string();
    res = res.add("://");
    if let Some(host) = url.host_str() {
        res = res.add(host);
        if let Some(port) = url.port() {
            res = res.add(format!(":{}", port).as_str());
        }
    }

    res = res.add(path.as_str());

    Ok(res)
}

impl FileSystemModuleLoader {
    pub fn new(base_path: &'static str) -> Self {
        Self { base_path }
    }

    fn get_real_fs_path(&self, abs_file_path: &str) -> String {
        assert!(abs_file_path.starts_with("file://"));
        format!("{}/{}", self.base_path, &abs_file_path[7..])
    }

    fn read_file(&self, filename: &str) -> std::io::Result<String> {
        let path = self.get_real_fs_path(filename);
        trace!("FileSystemModuleLoader::read_file -> {}", &path);

        fs::read_to_string(path)
    }

    fn file_exists(&self, filename: &str) -> bool {
        let path = self.get_real_fs_path(filename);
        trace!("FileSystemModuleLoader::file_exists -> {}", &path);
        Path::new(path.as_str()).exists()
    }
}

impl ScriptModuleLoader for FileSystemModuleLoader {
    fn normalize_path(&self, ref_path: &str, path: &str) -> Option<String> {
        // the ref path will always be an absolute path, so no need to parse . or ..
        // but even though we call it an absolute path here it will will be a relative path to the loader's main dir
        // so basically the file:// prefix is just to recognize the path a a path the FileSystemModuleLoader can handle

        if !ref_path.starts_with("file://") {
            return None;
        }
        if path.starts_with("file://") {
            return Some(path.to_string());
        }
        if path.contains("://") && !path.starts_with("file://") {
            // e.g. including a http:// based module from a file based module, should be handled by http loader
            return None;
        }

        let normalized = normalize_path(ref_path, path).ok().expect("parse failed");
        if self.file_exists(normalized.as_str()) {
            Some(normalized)
        } else {
            None
        }
    }

    fn load_module(&self, absolute_path: &str) -> String {
        self.read_file(absolute_path).unwrap_or("".to_string());
    }
}

pub struct HttpModuleLoader {
    _secure_only: bool,
    _allowed_domains: Option<Vec<String>>,
    _basic_auth: Option<(String, String)>,
    // todo stuff like clientcert / servercert checking
}

impl HttpModuleLoader {
    pub fn _new() -> Self {
        Self {
            _secure_only: false,
            _allowed_domains: None,
            _basic_auth: None,
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

#[cfg(test)]
mod tests {
    use crate::moduleloaders::{last_index_of, normalize_path};
    use quickjs_runtime::eserror::EsError;

    #[test]
    fn test_last_index_of() {
        assert_eq!(last_index_of("abcba", "b").unwrap(), 3);
        assert_eq!(last_index_of("abbcbba", "bb").unwrap(), 4);
    }

    #[test]
    fn test_normalize() {
        {
            assert_eq!(
                normalize_path("http://test.com/scripts/foo.es", "bar.mes")
                    .ok()
                    .unwrap(),
                "http://test.com/scripts/bar.mes"
            );
            assert_eq!(
                normalize_path("http://test.com/scripts/foo.es", "/bar.mes")
                    .ok()
                    .unwrap(),
                "http://test.com/bar.mes"
            );
            assert_eq!(
                normalize_path("http://test.com/scripts/foo.es", "../bar.mes")
                    .ok()
                    .unwrap(),
                "http://test.com/bar.mes"
            );
            assert_eq!(
                normalize_path("http://test.com/scripts/foo.es", "./bar.mes")
                    .ok()
                    .unwrap(),
                "http://test.com/scripts/bar.mes"
            );
        }
    }
}
