//! fetch api support
//!
//!
//!

use crate::features::js_fetch::proxies::RESPONSE_INSTANCES;
use hirofa_utils::js_utils::adapters::{JsRealmAdapter, JsValueAdapter};
use hirofa_utils::js_utils::JsError;
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
    pub fn from_js_object<R: JsRealmAdapter>(
        realm: &R,
        value: Option<&R::JsValueAdapterType>,
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
            realm.js_object_traverse_mut(init_obj, |prop_name, prop| {
                //

                match prop_name {
                    "method" => {
                        let val = prop.js_to_string()?;
                        fetch_init.method = Method::from_str(val.as_str())
                            .map_err(|_e| JsError::new_str("No such method"))?;
                    }
                    "mode" => {
                        let val = prop.js_to_string()?;
                        fetch_init.mode = Mode::from_str(val.as_str())
                            .map_err(|_e| JsError::new_str("No such mode"))?;
                    }
                    "cache" => {
                        let val = prop.js_to_string()?;
                        fetch_init.cache = Cache::from_str(val.as_str())
                            .map_err(|_e| JsError::new_str("No such cache"))?;
                    }
                    "credentials" => {
                        let val = prop.js_to_string()?;
                        fetch_init.credentials = Credentials::from_str(val.as_str())
                            .map_err(|_e| JsError::new_str("No such credentials"))?;
                    }

                    "redirect" => {
                        let val = prop.js_to_string()?;
                        fetch_init.redirect = Redirect::from_str(val.as_str())
                            .map_err(|_e| JsError::new_str("No such redirect"))?;
                    }

                    "body" => {
                        let val = prop.js_to_string()?;
                        fetch_init.body = Some(Body { text: val })
                    }
                    "headers" => {
                        realm.js_object_traverse_mut(prop, |header_name, header_val| {
                            fetch_init
                                .headers
                                .append(header_name, header_val.js_to_string()?.as_str());
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

pub struct Body {
    pub text: String,
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
    pub fn to_js_value<R: JsRealmAdapter>(
        self,
        realm: &R,
    ) -> Result<R::JsValueAdapterType, JsError> {
        // todo
        let inst_res = realm.js_proxy_instantiate(&[], "Response", &[])?;
        RESPONSE_INSTANCES.with(|rc| {
            let map = &mut *rc.borrow_mut();
            map.insert(inst_res.0, Arc::new(self))
        });
        Ok(inst_res.1)
    }
    pub async fn text(&self) -> Result<String, String> {
        let txt = self.body.text.clone(); // todo impl take in body
        Ok(txt)
    }
    pub async fn form_data(&self) -> Result<String, String> {
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
) -> Result<Response, String> {
    if let Some(url) = url {
        // todo cache reqwest client per realm_id

        let reqwest_resp = reqwest::get(url).await.map_err(|e| format!("{}", e))?;

        let response: Response = Response {
            body: Body {
                text: reqwest_resp.text().await.map_err(|e| format!("{}", e))?,
            },
            headers: Headers::new(),
            ok: false,
            redirected: false,
            status: 0,
            status_text: "",
            response_type: "",
            url: "".to_string(),
        };
        Ok(response)
    } else {
        Err("Missing mandatory url argument".to_string())
    }
}
