use hirofa_utils::js_utils::facades::JsRuntimeBuilder;

pub mod com;
pub mod db;
pub mod io;
pub mod lib;
pub mod util;

pub(crate) fn init<B: JsRuntimeBuilder>(builder: B) -> B {
    //com::init(builder);
    let builder = db::init(builder);
    let builder = io::init(builder);
    let builder = lib::init(builder);
    let builder = util::init(builder);
    builder
}
