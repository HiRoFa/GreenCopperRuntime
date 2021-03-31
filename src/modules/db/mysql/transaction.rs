use crate::modules::db::mysql::connection::MysqlConnection;
use quickjs_runtime::esvalue::{EsNullValue, EsValueConvertible, EsValueFacade};

pub(crate) struct _MysqlTransaction {
    con: mysql_lib::PooledConn,
}

impl _MysqlTransaction {
    fn _new(con: &MysqlConnection) -> Self {
        //
        let con = con.pool.get_conn().unwrap();

        Self { con }
    }
    fn _query(&self, _args: Vec<EsValueFacade>) -> EsValueFacade {
        EsNullValue {}.to_es_value_facade()
    }
    fn _commit(&self) -> EsValueFacade {
        EsNullValue {}.to_es_value_facade()
    }
    fn _rollback(&self) -> EsValueFacade {
        EsNullValue {}.to_es_value_facade()
    }
    fn _close(&self) -> EsValueFacade {
        EsNullValue {}.to_es_value_facade()
    }
}
