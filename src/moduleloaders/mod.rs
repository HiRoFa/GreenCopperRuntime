use hirofa_utils::js_utils::modules::ScriptModuleLoader;
use hirofa_utils::js_utils::JsError;
use log::trace;
use std::fs;
use std::ops::Add;
use std::path::{Path, PathBuf};
use url::Url;

pub struct FileSystemModuleLoader {
    base_path: PathBuf,
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
        x -= 1;
    }
    None
}

fn normalize_path(ref_path: &str, name: &str) -> Result<String, JsError> {
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
        .map_err(|e| JsError::new_string(format!("failed to parse Url: {}", e)))?;
    let path = if let Some(stripped) = name.strip_prefix('/') {
        stripped.to_string()
    } else {
        let url_path = url.path();
        if url_path.eq("/") {
            name.to_string()
        } else {
            format!("{}/{}", &url_path[1..], name)
        }
    };

    // remove ./
    // remove ..
    let mut path_parts: Vec<String> = path.split('/').into_iter().map(|s| s.to_string()).collect();

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
    res = res.add("/");

    res = res.add(path.as_str());

    Ok(res)
}

impl FileSystemModuleLoader {
    pub fn new(base_path: &'static str) -> Self {
        log::trace!("FileSystemModuleLoader::new {}", base_path);
        Self {
            base_path: Path::new(base_path).canonicalize().expect("path not found"),
        }
    }

    fn get_real_fs_path(&self, abs_file_path: &str) -> PathBuf {
        assert!(abs_file_path.starts_with("file:///"));
        self.base_path.join(Path::new(&abs_file_path[8..]))
    }

    fn read_file(&self, filename: &str) -> Result<String, String> {
        trace!("FileSystemModuleLoader::read_file -> {}", filename);

        let path = self.get_real_fs_path(filename);
        if !path.exists() {
            return Err(format!("File not found: {}", filename));
        }
        let path = path.canonicalize().unwrap();
        if !path.starts_with(&self.base_path) {
            return Err(format!("File not allowed: {}", filename));
        }

        fs::read_to_string(path)
            .map_err(|e| format!("failed to read: {}, caused by: {}", filename, e))
    }

    fn file_exists(&self, filename: &str) -> bool {
        trace!("FileSystemModuleLoader::file_exists -> {}", filename);
        let path = self.get_real_fs_path(filename);
        path.exists() && path.canonicalize().unwrap().starts_with(&self.base_path)
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

        match normalize_path(ref_path, path) {
            Ok(normalized) => {
                if self.file_exists(normalized.as_str()) {
                    Some(normalized)
                } else {
                    None
                }
            }
            Err(e) => {
                log::error!("could not normalize {}: {}", path, e);
                None
            }
        }
    }

    fn load_module(&self, absolute_path: &str) -> String {
        self.read_file(absolute_path)
            .unwrap_or_else(|_| "".to_string())
    }
}

#[cfg(any(feature = "all", feature = "com", feature = "http"))]
pub struct HttpModuleLoader {
    is_secure_only: bool,
    is_validate_content_type: bool,
    allowed_domains: Option<Vec<String>>,
    _basic_auth: Option<(String, String)>,
    // todo stuff like clientcert / servercert checking
}

#[cfg(any(feature = "all", feature = "com", feature = "http"))]
impl HttpModuleLoader {
    pub fn new() -> Self {
        Self {
            is_secure_only: false,
            is_validate_content_type: true,
            allowed_domains: None,
            _basic_auth: None,
        }
    }

    pub fn secure_only(mut self) -> Self {
        self.is_secure_only = true;
        self
    }

    pub fn validate_content_type(mut self, validate: bool) -> Self {
        self.is_validate_content_type = validate;
        self
    }

    pub fn allow_domain(mut self, domain: &str) -> Self {
        if self.allowed_domains.is_none() {
            self.allowed_domains = Some(vec![]);
        }
        let domains = self.allowed_domains.as_mut().unwrap();
        domains.push(domain.to_string());
        self
    }

    fn read_url(&self, url: &str) -> Option<String> {
        let resp = reqwest::blocking::get(url);
        //let req = reqwest::get(url);
        // todo make read_url async
        if resp.is_err() {
            return None;
        }
        let resp = resp.expect("wtf");
        if self.is_validate_content_type {
            let ct = &resp.headers()["Content-Type"];
            if !(ct.eq("application/javascript") || ct.eq("text/javascript")) {
                log::error!("loaded module {} did not have javascript Content-Type", url);
                return None;
            }
        }
        // todo async
        let res = resp.text();
        match res {
            Ok(script) => Some(script),
            Err(e) => {
                log::error!("could not load {} due to: {}", url, e);
                None
            }
        }
    }

    fn is_allowed(&self, absolute_path: &str) -> bool {
        if self.is_secure_only || self.allowed_domains.is_some() {
            match Url::parse(absolute_path) {
                Ok(url) => {
                    if self.is_secure_only && !url.scheme().eq("https") {
                        false
                    } else if let Some(domains) = &self.allowed_domains {
                        if let Some(host) = url.host_str() {
                            domains.contains(&host.to_string())
                        } else {
                            false
                        }
                    } else {
                        true
                    }
                }
                Err(e) => {
                    log::error!(
                        "HttpModuleLoader.is_allowed: could not parse url: {}, {}",
                        absolute_path,
                        e
                    );
                    false
                }
            }
        } else {
            true
        }
    }
}

#[cfg(any(feature = "all", feature = "com", feature = "http"))]
impl Default for HttpModuleLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(any(feature = "all", feature = "com", feature = "http"))]
impl ScriptModuleLoader for HttpModuleLoader {
    fn normalize_path(&self, ref_path: &str, path: &str) -> Option<String> {
        // the ref path will always be an absolute path

        if path.starts_with("http://") || path.starts_with("https://") {
            return if self.is_allowed(path) {
                Some(path.to_string())
            } else {
                None
            };
        }

        if path.contains("://") {
            return None;
        }

        if !(ref_path.starts_with("http://") || ref_path.starts_with("https://")) {
            return None;
        }

        match normalize_path(ref_path, path) {
            Ok(normalized) => {
                if self.is_allowed(normalized.as_str()) {
                    Some(normalized)
                } else {
                    None
                }
            }
            Err(e) => {
                log::error!("could not normalize: {}: {}", path, e);
                None
            }
        }
    }

    fn load_module(&self, absolute_path: &str) -> String {
        // todo, load_module should really return a Result
        if let Some(script) = self.read_url(absolute_path) {
            script
        } else {
            "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::moduleloaders::{
        last_index_of, normalize_path, FileSystemModuleLoader, HttpModuleLoader,
    };
    use hirofa_utils::js_utils::modules::ScriptModuleLoader;
    use std::path::Path;

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
            assert_eq!(
                normalize_path("file:///scripts/test.es", "bar.mes")
                    .ok()
                    .unwrap(),
                "file:///scripts/bar.mes"
            );
            assert_eq!(
                normalize_path("file:///scripts/test.es", "./bar.mes")
                    .ok()
                    .unwrap(),
                "file:///scripts/bar.mes"
            );
            assert_eq!(
                normalize_path("file:///scripts/test.es", "../bar.mes")
                    .ok()
                    .unwrap(),
                "file:///bar.mes"
            );
        }
    }

    #[test]
    fn test_http() {
        let loader = HttpModuleLoader::new()
            .secure_only()
            .validate_content_type(false)
            .allow_domain("github.com")
            .allow_domain("httpbin.org");
        // disallow http
        assert!(loader
            .normalize_path("http://github.com/example.js", "module.mjs")
            .is_none());
        // disallow domain
        assert!(loader
            .normalize_path("https://other.github.com/example.js", "module.mjs")
            .is_none());
        // allow domain
        assert!(loader
            .normalize_path("https://github.com/example.js", "module.mjs")
            .is_some());
        assert_eq!(
            loader
                .normalize_path("https://github.com/scripts/example.js", "module.mjs")
                .unwrap(),
            "https://github.com/scripts/module.mjs"
        );
        assert_eq!(
            loader
                .normalize_path("https://github.com/example.js", "module.mjs")
                .unwrap(),
            "https://github.com/module.mjs"
        );
    }

    #[test]
    fn test_fs() {
        let loader = FileSystemModuleLoader::new("./modules");
        let path = Path::new("./modules").canonicalize().unwrap();
        println!("path = {:?}", path);
        assert!(loader
            .normalize_path("file:///test.es", "utils/assertions.mes")
            .is_some());
        assert!(loader
            .normalize_path("file:///test.es", "utils/notfound.mes")
            .is_none());
    }
}
