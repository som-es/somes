use dataservice::db::{
    models::{DbDelegate, DbLegislativeInitiative, DbProposal, DbProposalQuery},
    schema::{
        delegates::dsl::delegates,
        legislative_initiatives::{created_at, dsl::legislative_initiatives},
        proposals::dsl::proposals,
    },
};
use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};

use crate::{routes::RequestFilter, DATASERVICE_URL};

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

pub fn get_legislative_initiatives(con: &mut PgConnection, filter: RequestFilter) -> QueryResult<Vec<DbLegislativeInitiative>> {
    legislative_initiatives
        .filter(created_at.gt(filter.start))
        .filter(created_at.lt(filter.end))
        .load::<DbLegislativeInitiative>(con)
}

#[cfg(test)]
mod tests {
    use crate::{dataservice::dataservice_con, routes::RequestFilter, today_and_time};

    use super::{get_delegates, get_legislative_initiatives};

    #[test]
    fn test_get_delegates() {
        let con = &mut dataservice_con();
        let delegates = get_delegates(con);
        println!("delegates: {delegates:?}");
    }

    #[test]
    fn test_get_legislative_inits() {
        let con = &mut dataservice_con();
        let filter = RequestFilter {
            start: today_and_time() - chrono::Duration::days(7),
            end: today_and_time(),
        };
        let legis_inits = get_legislative_initiatives(con, filter);
        println!("legis_inits: {legis_inits:?}");
    }
}
