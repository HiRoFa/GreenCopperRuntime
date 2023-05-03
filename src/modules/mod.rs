use quickjs_runtime::builder::QuickJsRuntimeBuilder;

pub mod com;
#[cfg(any(feature = "all", feature = "crypto"))]
pub mod crypto;
pub mod db;
pub mod io;
#[cfg(any(feature = "all", feature = "jwt"))]
pub mod jwt;
pub mod lib;
#[cfg(any(feature = "all", feature = "parsers"))]
pub mod parsers;
pub mod util;

#[cfg(any(feature = "all", feature = "encoding"))]
pub mod encoding;

#[cfg(any(feature = "all", feature = "htmldom"))]
pub mod htmldom;

pub(crate) fn init(builder: QuickJsRuntimeBuilder) -> QuickJsRuntimeBuilder {
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
    #[cfg(any(feature = "all", feature = "encoding"))]
    let builder = encoding::init(builder);
    util::init(builder)
}
