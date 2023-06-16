use dataservice::db::{
    models::{DbDelegate, DbLegislativeInitiativeQuery, DbProposalQuery, DbSpeech, DbVote},
    schema::{
        delegates::{council, dsl::delegates, is_active, seat_row},
        legislative_initiatives::{created_at, dsl::legislative_initiatives},
        proposals::dsl::proposals,
        speeches::dsl::speeches,
        votes::{dsl::votes, legislative_initiatives_id},
    },
};
use diesel::{
    sql_query, Connection, ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use crate::{model::SpeakerByHours, routes::RequestFilter, today, DATASERVICE_URL};

/*#[derive(QueryableByName, PartialEq, Eq)]
struct User {
    name: String,
    party: String,
    // hours_spoken: f32
}*/

// MIND: the resulting hours are not relative to the total tenure (amtzeit)
pub fn get_speakers_by_hours(con: &mut PgConnection) -> QueryResult<Vec<SpeakerByHours>> {
    sql_query(
        "select 
        delegates.name, 
        delegates.image_url,
        delegates.party, 
        SUM(
        plenar_speeches.duration_in_seconds
        ) / (60.0 * 60.0) AS hours_spoken 
    from 
        plenar_speeches 
        inner join debates on plenar_speeches.debate_id = debates.id 
        inner join plenar_infos on debates.plenar_id = plenar_infos.id 
        inner join delegates on delegates.id = plenar_speeches.delegate_id 
    where 
        plenar_infos.legislative_period = 'XXVII' 
        and delegates.council = 'nr'
    group by 
        plenar_speeches.delegate_id, 
        delegates.name, 
        delegates.party, 
        delegates.council 
    order by 
        hours_spoken DESC;",
    )
    .load::<SpeakerByHours>(con)
}

pub fn dataservice_con() -> PgConnection {
    PgConnection::establish(DATASERVICE_URL)
        .expect("Can't establish dataservice database conntection.")
}

pub fn get_delegates(con: &mut PgConnection) -> QueryResult<Vec<DbDelegate>> {
    delegates
        .filter(is_active.eq(true))
        .filter(council.eq("nr"))
        .filter(seat_row.is_not_null())
        .load(con)
    // delegates.load(con)
}

pub fn get_proposals(con: &mut PgConnection) -> QueryResult<Vec<DbProposalQuery>> {
    proposals.load::<DbProposalQuery>(con)
}

pub fn get_legislative_initiatives(
    con: &mut PgConnection,
    filter: RequestFilter,
) -> QueryResult<Vec<DbLegislativeInitiativeQuery>> {
    legislative_initiatives
        .filter(created_at.gt(filter.start))
        .filter(created_at.lt(filter.end))
        .load::<DbLegislativeInitiativeQuery>(con)
}

pub fn get_latest_legislative_initiatives(
    con: &mut PgConnection,
) -> QueryResult<Vec<DbLegislativeInitiativeQuery>> {
    legislative_initiatives
        .filter(created_at.gt(today() - chrono::Duration::days(30)))
        .filter(created_at.lt(today()))
        .order(created_at.desc())
        .load::<DbLegislativeInitiativeQuery>(con)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VoteResult {
    pub legislative_initiative: DbLegislativeInitiativeQuery,
    pub votes: Vec<DbVote>,
    pub speeches: Vec<DbSpeech>,
}

pub fn get_latest_vote_results(con: &mut PgConnection) -> QueryResult<Vec<VoteResult>> {
    get_latest_legislative_initiatives(con)?
        .into_iter()
        .map(|legis_init| {
            Ok(VoteResult {
                votes: get_votes_from_legis_init(con, legis_init.id)?,
                speeches: get_speeches_from_legis_init(con, legis_init.id)?,
                legislative_initiative: legis_init,
            })
        })
        .collect::<QueryResult<Vec<VoteResult>>>()
}

pub fn get_votes_from_legis_init(
    con: &mut PgConnection,
    legis_init_id: i32,
) -> QueryResult<Vec<DbVote>> {
    votes
        .filter(legislative_initiatives_id.eq(legis_init_id))
        .load::<DbVote>(con)
}

pub fn get_speeches_from_legis_init(
    con: &mut PgConnection,
    legis_init_id: i32,
) -> QueryResult<Vec<DbSpeech>> {
    speeches
        .filter(dataservice::db::schema::speeches::legislative_initiatives_id.eq(legis_init_id))
        .load::<DbSpeech>(con)
}

#[cfg(test)]
mod tests {
    use crate::{dataservice::dataservice_con, routes::RequestFilter, today};

    use super::{get_delegates, get_latest_vote_results, get_legislative_initiatives};

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
            start: today() - chrono::Duration::days(7),
            end: today(),
        };
        let legis_inits = get_legislative_initiatives(con, filter);
        println!("legis_inits: {legis_inits:?}");
    }

    #[test]
    fn test_get_combined_latest_votes_and_legis_inits() {
        let con = &mut dataservice_con();
        let res = get_latest_vote_results(con);
        println!("res: {res:?}");
    }
}
