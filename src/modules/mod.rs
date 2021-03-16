use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;

pub mod com;
pub mod db;
pub mod io;
pub mod lib;
pub mod util;

pub(crate) fn init(builder: EsRuntimeBuilder) -> EsRuntimeBuilder {
    let builder = com::init(builder);
    let builder = db::init(builder);
    let builder = io::init(builder);
    let builder = lib::init(builder);
    util::init(builder)
}
