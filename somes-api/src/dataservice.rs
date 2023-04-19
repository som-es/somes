use dataservice::db::{models::{DbDelegate, DbProposal, DbProposalQuery}, schema::{delegates::dsl::delegates, proposals::dsl::proposals}};
use diesel::{Connection, PgConnection, QueryResult, RunQueryDsl};

use crate::DATASERVICE_URL;

pub fn dataservice_con() -> PgConnection {
    PgConnection::establish(DATASERVICE_URL)
        .expect("Can't establish dataservice database conntection.")
}

pub fn get_delegates(con: &mut PgConnection) -> QueryResult<Vec<DbDelegate>> {
    delegates.load(con)
}

pub fn get_proposals(con: &mut PgConnection) -> QueryResult<Vec<DbProposalQuery>> {
    proposals.load::<DbProposalQuery>(con)
}

#[cfg(test)]
mod tests {
    use crate::{dataservice::dataservice_con, establish_connection};

    use super::get_delegates;

    #[test]
    fn test_get_delegates() {
        let con = &mut dataservice_con();
        let delegates = get_delegates(con);
        println!("delegates: {delegates:?}");
    }
}
