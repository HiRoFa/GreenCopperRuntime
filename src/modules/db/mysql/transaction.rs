use crate::modules::db::mysql::connection::MysqlConnection;
use hirofa_utils::js_utils::facades::values::JsValueFacade;

pub(crate) struct _MysqlTransaction {
    con: mysql_lib::PooledConn,
}

impl _MysqlTransaction {
    fn _new(con: &MysqlConnection) -> Self {
        //
        let con = con.pool.get_conn().unwrap();

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
