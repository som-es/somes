use dataservice::db::{models::DbDelegate, schema::delegates::dsl::delegates};
use diesel::{Connection, PgConnection, QueryResult, RunQueryDsl};

use crate::DATASERVICE_URL;

pub fn dataservice_con() -> PgConnection {
    PgConnection::establish(DATASERVICE_URL)
        .expect("Can't establish dataservice database conntection.")
}

pub fn get_delegates(con: &mut PgConnection) -> QueryResult<Vec<DbDelegate>> {
    delegates.load(con)
}
