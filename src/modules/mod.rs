use crate::GrecoRuntimeBuilder;
use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;
use quickjs_runtime::quickjsruntime::ModuleLoader;
use std::collections::HashMap;

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
    let builder = util::init(builder);
    builder
}
