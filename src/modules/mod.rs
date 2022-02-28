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

pub(crate) fn init<B: JsRuntimeBuilder>(builder: B) -> B {
    //com::init(builder);
    let builder = db::init(builder);
    let builder = io::init(builder);
    let builder = lib::init(builder);
    #[cfg(any(feature = "all", feature = "crypto"))]
    let builder = crypto::init(builder);
    #[cfg(any(feature = "all", feature = "jwt"))]
    let builder = jwt::init(builder);
    util::init(builder)
}
