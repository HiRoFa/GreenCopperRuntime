use crate::modules::db::mysql::connection::MysqlConnection;
use futures::executor::block_on;
use hirofa_utils::js_utils::facades::values::JsValueFacade;
use mysql_lib::Conn;

pub(crate) struct _MysqlTransaction {
    con: Conn,
}

impl _MysqlTransaction {
    fn _new(con: &MysqlConnection) -> Self {
        //
        // todo async constructor? errors handling
        let con = block_on(con.pool.get_pool().get_conn()).unwrap();

        Self { con }
    }
    fn _query(&self, _args: &[JsValueFacade]) -> JsValueFacade {
        JsValueFacade::Null
    }
    fn _commit(&self) -> JsValueFacade {
        JsValueFacade::Null
    }
    fn _rollback(&self) -> JsValueFacade {
        JsValueFacade::Null
    }
    fn _close(&self) -> JsValueFacade {
        JsValueFacade::Null
    }
}
