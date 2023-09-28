//! fetch api support
//!
//!
//!

use crate::features::js_fetch::proxies::RESPONSE_INSTANCES;
use quickjs_runtime::jsutils::JsError;
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use quickjs_runtime::quickjsvalueadapter::QuickJsValueAdapter;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

// todo see stackoverflow.com/questions/44121783
pub enum Mode {
    Cors,
    NoCors,
    SameOrigin,
}

impl Mode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Cors => "cors",
            Self::NoCors => "no-cors",
            Self::SameOrigin => "same-origin",
        }
    }
}

impl FromStr for Mode {
    type Err = ();

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        match val {
            "cors" => Ok(Self::Cors),
            "no-cors" => Ok(Self::NoCors),
            "same-origin" => Ok(Self::SameOrigin),
            _ => Err(()),
        }
    }
}

pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
    Copy,
    Lock,
    Mkcol,
    Move,
    Propfind,
    Proppatch,
    Unlock,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
            Method::Patch => "PATCH",
            Method::Copy => "COPY",
            Method::Lock => "LOCK",
            Method::Mkcol => "MKCOL",
            Method::Move => "MOVE",
            Method::Propfind => "PROPFIND",
            Method::Proppatch => "PROPPATCH",
            Method::Unlock => "UNLOCK",
        }
    }
}

impl FromStr for Method {
    type Err = ();

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        match val.to_ascii_uppercase().as_str() {
            "GET" => Ok(Self::Get),
            "HEAD" => Ok(Self::Head),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            "CONNECT" => Ok(Self::Connect),
            "OPTIONS" => Ok(Self::Options),
            "TRACE" => Ok(Self::Trace),
            "PATCH" => Ok(Self::Patch),
            "COPY" => Ok(Self::Copy),
            "LOCK" => Ok(Self::Lock),
            "MKCOL" => Ok(Self::Mkcol),
            "MOVE" => Ok(Self::Move),
            "PROPFIND" => Ok(Self::Propfind),
            "PROPPATCH" => Ok(Self::Proppatch),
            "UNLOCK" => Ok(Self::Unlock),

            _ => Err(()),
        }
    }
}

pub enum Redirect {
    Follow,
    Manual,
    Error,
}

impl Redirect {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Follow => "follow",
            Self::Manual => "manual",
            Self::Error => "error",
        }
    }
}

impl FromStr for Redirect {
    type Err = ();

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        match val {
            "manual" => Ok(Self::Manual),
            "follow" => Ok(Self::Follow),
            "error" => Ok(Self::Error),
            _ => Err(()),
        }
    }
}

pub enum Credentials {
    Omit,
    SameOrigin,
    Include,
}

impl Credentials {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Omit => "omit",
            Self::SameOrigin => "same-origin",
            Self::Include => "include",
        }
    }
}

impl FromStr for Credentials {
    type Err = ();

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        match val {
            "omit" => Ok(Self::Omit),
            "same-origin" => Ok(Self::SameOrigin),
            "include" => Ok(Self::Include),
            _ => Err(()),
        }
    }
}

pub enum Cache {
    Default,
    NoStore,
    Reload,
    NoCache,
    ForceCache,
    OnlyIfCached,
}

impl Cache {
    pub fn as_str(&self) -> &'static str {
        match self {
            Cache::Default => "default",
            Cache::NoStore => "no-store",
            Cache::Reload => "reload",
            Cache::NoCache => "no-cache",
            Cache::ForceCache => "force-cache",
            Cache::OnlyIfCached => "only-if-cached",
        }
    }
}

impl FromStr for Cache {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Self::Default),
            "no-store" => Ok(Self::NoStore),
            "reload" => Ok(Self::Reload),
            "no-cache" => Ok(Self::NoCache),
            "force-cache" => Ok(Self::ForceCache),
            "only-if-cached" => Ok(Self::OnlyIfCached),
            _ => Err(()),
        }
    }
}

pub struct FetchInit {
    method: Method,
    headers: Headers,
    body: Option<Body>,
    mode: Mode,
    credentials: Credentials,
    cache: Cache,
    redirect: Redirect,
}
impl FetchInit {
    pub fn from_js_object(
        realm: &QuickJsRealmAdapter,
        value: Option<&QuickJsValueAdapter>,
    ) -> Result<Self, JsError> {
        let mut fetch_init = Self {
            method: Method::Get,
            headers: Headers::new(),
            body: None,
            mode: Mode::NoCors,
            credentials: Credentials::SameOrigin,
            cache: Cache::Default,
            redirect: Redirect::Follow,
        };

        if let Some(init_obj) = value {
            realm.traverse_object_mut(init_obj, |prop_name, prop| {
                //

                match prop_name {
                    "method" => {
                        let val = prop.to_string()?;
                        fetch_init.method = Method::from_str(val.as_str())
                            .map_err(|_e| JsError::new_str("No such method"))?;
                    }
                    "mode" => {
                        let val = prop.to_string()?;
                        fetch_init.mode = Mode::from_str(val.as_str())
                            .map_err(|_e| JsError::new_str("No such mode"))?;
                    }
                    "cache" => {
                        let val = prop.to_string()?;
                        fetch_init.cache = Cache::from_str(val.as_str())
                            .map_err(|_e| JsError::new_str("No such cache"))?;
                    }
                    "credentials" => {
                        let val = prop.to_string()?;
                        fetch_init.credentials = Credentials::from_str(val.as_str())
                            .map_err(|_e| JsError::new_str("No such credentials"))?;
                    }

                    "redirect" => {
                        let val = prop.to_string()?;
                        fetch_init.redirect = Redirect::from_str(val.as_str())
                            .map_err(|_e| JsError::new_str("No such redirect"))?;
                    }

                    "body" => {
                        if prop.is_string() {
                            let val = prop.to_string()?;
                            fetch_init.body = Some(Body {
                                text: Some(val),
                                bytes: None,
                            });
                        }
                        if prop.is_typed_array() {
                            let val = realm.copy_typed_array_buffer(prop)?;
                            fetch_init.body = Some(Body {
                                bytes: Some(val),
                                text: None,
                            });
                        }
                    }
                    "headers" => {
                        realm.traverse_object_mut(prop, |header_name, header_val| {
                            fetch_init
                                .headers
                                .append(header_name, header_val.to_string()?.as_str());
                            Ok(())
                        })?;
                    }

                    _ => {}
                }

                Ok(())
            })?;
        }
        Ok(fetch_init)
    }
}

pub struct Headers {
    map: HashMap<String, Vec<String>>,
}
impl Headers {
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }
    pub fn append(&mut self, name: &str, value: &str) {
        if !self.map.contains_key(name) {
            self.map.insert(name.to_string(), vec![]);
        }
        let vec = self.map.get_mut(name).unwrap();
        vec.push(value.to_string());
    }
}
impl Default for Headers {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Body {
    pub text: Option<String>,
    pub bytes: Option<Vec<u8>>,
}
impl Body {
    //
}

pub struct Response {
    pub body: Body,
    pub headers: Headers,
    pub ok: bool,
    pub redirected: bool,
    pub status: u16,
    pub status_text: &'static str,
    pub response_type: &'static str,
    pub url: String,
}
impl Response {
    pub fn to_js_value(self, realm: &QuickJsRealmAdapter) -> Result<QuickJsValueAdapter, JsError> {
        // todo
        let inst_res = realm.instantiate_proxy(&[], "Response", &[])?;
        RESPONSE_INSTANCES.with(|rc| {
            let map = &mut *rc.borrow_mut();
            map.insert(inst_res.0, Arc::new(self))
        });
        Ok(inst_res.1)
    }
    pub async fn text(&self) -> Result<String, JsError> {
        if let Some(text) = self.body.text.as_ref() {
            Ok(text.clone())
        } else if let Some(bytes) = self.body.bytes.as_ref() {
            Ok(String::from_utf8(bytes.clone())
                .map_err(|_e| JsError::new_str("could not convert to string (utf8 error)"))?)
        } else {
            Err(JsError::new_str("body had no content"))
        }
    }
    // todo impl some sort of take so we don;t copy bytes every time they are used (ReadableStream and such)
    pub async fn bytes(&self) -> Result<Vec<u8>, JsError> {
        if let Some(bytes) = self.body.bytes.as_ref() {
            Ok(bytes.clone())
        } else {
            Err(JsError::new_str("body had no content"))
        }
    }
    pub async fn form_data(&self) -> Result<String, JsError> {
        todo!()
    }
}

pub trait Request {
    fn get_url(&self) -> &str;
    fn get_header(&self, name: &str) -> &[String];
}

pub async fn do_fetch(
    _realm_id: String,
    url: Option<String>,
    fetch_init: FetchInit,
) -> Result<Response, JsError> {
    if let Some(url) = url {
        // todo cache reqwest client per realm_id

        let client = reqwest::ClientBuilder::new()
            .build()
            .map_err(|e| JsError::new_string(format!("{e}")))?;
        let method = reqwest::Method::from_str(fetch_init.method.as_str())
            .map_err(|e| JsError::new_string(format!("{e}")))?;

        let mut request = client.request(method, url);

        if let Some(body) = fetch_init.body {
            if let Some(text) = body.text.as_ref() {
                request = request.body(text.clone());
            } else if let Some(bytes) = body.bytes.as_ref() {
                request = request.body(bytes.clone()); // todo impl .take
            }
        }

        for header in &fetch_init.headers.map {
            for val in header.1 {
                request = request.header(header.0, val);
            }
        }

        let response_fut = request.send();

        let reqwest_resp = response_fut
            .await
            .map_err(|e| JsError::new_string(format!("{e}")))?;

        let ok = reqwest_resp.status().is_success();
        let status = reqwest_resp.status().as_u16();

        let mut is_text = false;
        if let Some(ct) = reqwest_resp.headers().get("content-type") {
            let ct_str = ct
                .to_str()
                .map_err(|e| JsError::new_string(format!("{}", e)))?;
            if ct_str.eq("text/plain")
                || ct_str.eq("text/html")
                || ct_str.eq("application/json")
                || ct_str.eq("image/svg+xml")
            {
                is_text = true;
            }
        }

        let body = if is_text {
            Body {
                // todo support bytes, it would make more sense to make reqwest_resp a member of reponse, then we can also impl Response.arrayBuffer() or Response.blob()
                text: Some(
                    reqwest_resp
                        .text()
                        .await
                        .map_err(|e| JsError::new_string(format!("{e}")))?,
                ),
                bytes: None,
            }
        } else {
            let bytes: Vec<u8> = reqwest_resp
                .bytes()
                .await
                .map_err(|e| JsError::new_string(format!("{}", e)))?
                .to_vec();

            Body {
                text: None,
                bytes: Some(bytes),
            }
        };

        let response: Response = Response {
            body,
            headers: Headers::new(),
            ok,
            redirected: false,
            status,
            status_text: "",
            response_type: "",
            url: "".to_string(),
        };
        Ok(response)
    } else {
        Err(JsError::new_str("Missing mandatory url argument"))
    }
}

#[cfg(test)]
pub mod tests {
    /*
    use futures::executor::block_on;
    use quickjs_runtime::builder::QuickJsRuntimeBuilder;
    use quickjs_runtime::jsutils::Script;
    use quickjs_runtime::values::JsValueFacade;

    #[test]
    fn test_fetch_1() {
        let rt = crate::init_greco_rt(QuickJsRuntimeBuilder::new()).build();
        let mut res = block_on(rt.eval(None, Script::new("test_fetch_1.js", r#"
            (async () => {
                let res = await fetch("https://httpbin.org/post", {method: "POST", headers:{"Content-Type": "application/json"}, body: JSON.stringify({obj: 1})});
                return res.text();
            })();
        "#))).expect("script failed");
        if let JsValueFacade::JsPromise { cached_promise } = res {
            res = block_on(cached_promise.get_promise_result())
                .expect("promise timed out")
                .expect("promise failed");
        }

        let str = res.stringify();

        println!("res: {str}");

        assert!(str.contains("\"json\": {\n    \"obj\": 1\n  }"))
    }*/
}
