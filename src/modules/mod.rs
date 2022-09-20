use hirofa_utils::js_utils::facades::JsRuntimeBuilder;

pub mod com;
#[cfg(any(feature = "all", feature = "crypto"))]
pub mod crypto;
pub mod db;
pub mod io;
#[cfg(any(feature = "all", feature = "jwt"))]
pub mod jwt;
pub mod lib;
pub mod util;
#[cfg(any(feature = "all", feature = "parsers"))]
pub mod parsers;

#[cfg(any(feature = "all", feature = "htmldom"))]
pub mod htmldom;

pub(crate) fn init<B: JsRuntimeBuilder>(builder: B) -> B {
    //com::init(builder);
    let builder = db::init(builder);
    let builder = io::init(builder);
    let builder = lib::init(builder);
    #[cfg(any(feature = "all", feature = "crypto"))]
    let builder = crypto::init(builder);
    #[cfg(any(feature = "all", feature = "jwt"))]
    let builder = jwt::init(builder);
    #[cfg(any(feature = "all", feature = "htmldom"))]
    let builder = htmldom::init(builder);
    #[cfg(any(feature = "all", feature = "parsers"))]
        let builder = parsers::init(builder);
    util::init(builder)
}
