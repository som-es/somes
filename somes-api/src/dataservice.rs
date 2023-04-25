use dataservice::db::{
    models::{DbDelegate, DbLegislativeInitiative, DbProposalQuery, DbVote},
    schema::{
        delegates::dsl::delegates,
        legislative_initiatives::{created_at, dsl::legislative_initiatives},
        proposals::dsl::proposals,
        votes::{dsl::votes, legislative_initiatives_id},
    },
};
use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::{routes::RequestFilter, today_and_time, DATASERVICE_URL};

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

pub fn get_legislative_initiatives(
    con: &mut PgConnection,
    filter: RequestFilter,
) -> QueryResult<Vec<DbLegislativeInitiative>> {
    legislative_initiatives
        .filter(created_at.gt(filter.start))
        .filter(created_at.lt(filter.end))
        .load::<DbLegislativeInitiative>(con)
}

pub fn get_latest_legislative_initiatives(
    con: &mut PgConnection,
) -> QueryResult<Vec<DbLegislativeInitiative>> {
    legislative_initiatives
        .filter(created_at.gt(today_and_time() - chrono::Duration::days(7)))
        .filter(created_at.lt(today_and_time()))
        .load::<DbLegislativeInitiative>(con)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VoteResult {
    pub legislative_initiative: DbLegislativeInitiative,
    pub votes: Vec<DbVote>,
}

pub fn get_latest_legislative_initiatives_and_votes(
    con: &mut PgConnection,
) -> QueryResult<Vec<VoteResult>> {
    get_latest_legislative_initiatives(con)?
        .into_iter()
        .map(|legis_init| Ok(
            VoteResult {
                votes: get_votes_from_legis_init(con, &legis_init.id)?,
                legislative_initiative: legis_init,
            }))
        .collect::<QueryResult<Vec<VoteResult>>>()    
}

pub fn get_votes_from_legis_init(
    con: &mut PgConnection,
    legis_init: &str,
) -> QueryResult<Vec<DbVote>> {
    votes
        .filter(legislative_initiatives_id.eq(legis_init))
        .load::<DbVote>(con)
}

mod tests {
    use crate::{dataservice::dataservice_con, routes::RequestFilter, today_and_time};

    use super::{get_delegates, get_legislative_initiatives, get_latest_legislative_initiatives_and_votes};

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

    #[test]
    fn test_get_combined_latest_votes_and_legis_inits() {
        let con = &mut dataservice_con();
        let res = get_latest_legislative_initiatives_and_votes(con);
        println!("res: {res:?}");
    }
}
