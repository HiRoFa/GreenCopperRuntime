//! fetch api support
//!
//!
//!

use hirofa_utils::js_utils::adapters::JsRealmAdapter;
use hirofa_utils::js_utils::JsError;
use std::str::FromStr;

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
    headers: Option<Headers>,
    body: Option<Body>,
    mode: Option<Mode>,
    credentials: Option<Credentials>,
    cache: Option<Cache>,
}
impl FetchInit {
    pub fn from_js_object<R: JsRealmAdapter>(
        realm: &R,
        value: Option<&R::JsValueAdapterType>,
    ) -> Self {
        let ret = Self {
            method: Method::Get,
            headers: None,
            body: None,
            mode: None,
            credentials: None,
            cache: None,
        };

        ret
    }
}

pub struct Headers {}
impl Headers {
    pub fn append(&mut self, name: &str, value: &str) {
        todo!()
    }
}

pub struct Body {}
impl Body {
    //
}

pub struct Response {}
impl Response {
    pub fn to_js_value<R: JsRealmAdapter>(
        &self,
        realm: &R,
    ) -> Result<R::JsValueAdapterType, JsError> {
        realm.js_null_create()
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
        todo!()
    } else {
        Err("Missing mandatory url argument".to_string())
    }
}
