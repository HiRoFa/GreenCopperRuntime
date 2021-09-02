use hirofa_utils::js_utils::facades::JsRuntimeBuilder;

pub mod com;
pub mod db;
pub mod io;
pub mod lib;
pub mod util;

pub(crate) fn init<B: JsRuntimeBuilder>(builder: &mut B) {
    //com::init(builder);
    db::init(builder);
    io::init(builder);
    lib::init(builder);
    util::init(builder)
}
