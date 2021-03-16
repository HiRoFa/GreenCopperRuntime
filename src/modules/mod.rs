use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;

mod com;
mod db;
mod io;
mod lib;
mod util;

pub fn init(builder: EsRuntimeBuilder) -> EsRuntimeBuilder {
    let builder = com::init(builder);
    let builder = db::init(builder);
    let builder = io::init(builder);
    let builder = lib::init(builder);
    util::init(builder)
}
