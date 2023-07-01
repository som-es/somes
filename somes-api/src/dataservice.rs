use dataservice::db::{
    models::{
        DbDelegate, DbLegislativeInitiativeQuery, DbParty, DbProposalQuery, DbSpeech, DbVote,
    },
    schema::{
        delegates::{council, dsl::delegates, is_active, seat_row},
        legislative_initiatives::{accepted, created_at, dsl::legislative_initiatives},
        parties::dsl::parties,
        proposals::dsl::proposals,
        speeches::dsl::speeches,
        votes::{dsl::votes, legislative_initiatives_id},
    },
};
use diesel::{
    sql_query, sql_types::Text, Connection, ExpressionMethods, PgConnection, QueryDsl, QueryResult,
    RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use crate::{
    model::{CallToOrdersPerPartyDelegates, DelegateByCallToOrders, SpeakerByHours},
    routes::{LegisPeriod, RequestFilter},
    today, DATASERVICE_URL,
};

// TODO: move all dataservice.rs function to the dataservice library

/*#[derive(QueryableByName, PartialEq, Eq)]
struct User {
    name: String,
    party: String,
    // hours_spoken: f32
}*/

pub fn get_call_to_orders_per_party_delegates(
    con: &mut PgConnection,
) -> QueryResult<Vec<CallToOrdersPerPartyDelegates>> {
    sql_query(
        "select 
    delegates.party, 
    COUNT(*)::Integer as call_to_order_amount, 
    delegates_in_party.delegate_amount, 
    (
      COUNT(*):: real / delegates_in_party.delegate_amount
    )::real as ratio
  from 
    call_to_order 
    inner join delegates on delegates.id = call_to_order.receiver_id 
    join (
      select 
        party, 
        COUNT(*)::Integer as delegate_amount
      from 
        delegates 
      where 
        is_active = 't' 
        and council = 'nr' 
        and seat_row is not null 
      group by 
        party
    ) as delegates_in_party on delegates_in_party.party = delegates.party 
  group by 
    delegates.party, 
    delegates_in_party.delegate_amount
  order by
    ratio DESC;
  ",
    )
    .load(con)
}

/// ```sql
/// select
/// delegates.name,
/// delegates.image_url,
/// delegates.party,
/// cast(COUNT(*) as Integer) as call_to_order_amount
/// from call_to_order
/// inner join delegates on call_to_order.receiver_id=delegates.id
/// inner join plenar_infos on call_to_order.plenar_id = plenar_infos.id
/// where plenar_infos.legislative_period = 'XXVI'
/// group by delegates.name,delegates.image_url,delegates.party,call_to_order.receiver_id
/// order by call_to_order_amount DESC;
/// ```
pub fn get_delegates_by_call_to_orders_by_legis_period(
    con: &mut PgConnection,
    legis_period: &LegisPeriod,
) -> QueryResult<Vec<DelegateByCallToOrders>> {
    sql_query(
        "select 
    delegates.name,
    delegates.image_url,
    delegates.party,
    cast(COUNT(*) as Integer) as call_to_order_amount
    from call_to_order 
    inner join delegates on call_to_order.receiver_id=delegates.id 
    inner join plenar_infos on call_to_order.plenar_id = plenar_infos.id
    where plenar_infos.legislative_period = $1
    group by delegates.name,delegates.image_url,delegates.party,call_to_order.receiver_id 
    order by call_to_order_amount DESC;",
    )
    .bind::<Text, _>(&legis_period.period)
    .load(con)
    /*
    use dataservice::db::schema::call_to_order::dsl::call_to_order;
    use dataservice::db::schema::plenar_infos::dsl::plenar_infos;

    use dataservice::db::schema::plenar_infos::*;
    use dataservice::db::schema::delegates::{name, image_url, party};
    use dataservice::db::schema::call_to_order::receiver_id;

    call_to_order.inner_join(delegates)
        .inner_join(plenar_infos)
        .filter(legislative_period.eq(legis_period.period))
        .group_by(name)
        .group_by(image_url)
        .group_by(party)
        .group_by(receiver_id)
        .order_by(receiver_id)
        .select((
            name,
            image_url,
            party,
            diesel::dsl::count(receiver_id).r#as("call_to_order_amount"),
        ))
        .load::<DelegateByCallToOrders>(con);*/
}

pub fn get_delegates_by_call_to_orders(
    con: &mut PgConnection,
) -> QueryResult<Vec<DelegateByCallToOrders>> {
    sql_query(
        "select 
        delegates.name,
        delegates.image_url,
        delegates.party,
        cast(COUNT(*) as Integer) as call_to_order_amount
        from call_to_order 
        inner join delegates on call_to_order.receiver_id=delegates.id 
        group by delegates.name,delegates.image_url,delegates.party,call_to_order.receiver_id 
        order by call_to_order_amount DESC;",
    )
    .load(con)
}

pub fn get_speakers_by_hours(con: &mut PgConnection) -> QueryResult<Vec<SpeakerByHours>> {
    sql_query(
        "select 
    delegates.name, 
    delegates.image_url,
    delegates.party, 
    cast(SUM(
    plenar_speeches.duration_in_seconds
    ) / (60.0 * 60.0) as real) AS hours_spoken 
from 
    plenar_speeches 
    inner join debates on plenar_speeches.debate_id = debates.id 
    inner join plenar_infos on debates.plenar_id = plenar_infos.id 
    inner join delegates on delegates.id = plenar_speeches.delegate_id 
group by 
    plenar_speeches.delegate_id, 
    delegates.image_url, 
    delegates.name, 
    delegates.party, 
    delegates.council 
order by 
    hours_spoken DESC;",
    )
    .load(con)
}

// MIND: the resulting hours are not relative to the total tenure (amtzeit)
pub fn get_speakers_by_hours_by_legis_period(
    con: &mut PgConnection,
    legis_period: &LegisPeriod,
) -> QueryResult<Vec<SpeakerByHours>> {
    // mind "real" datatype -> float
    sql_query(
        "select 
        delegates.name, 
        delegates.image_url,
        delegates.party, 
        cast(SUM(
        plenar_speeches.duration_in_seconds
        ) / (60.0 * 60.0) as real) AS hours_spoken 
    from 
        plenar_speeches 
        inner join debates on plenar_speeches.debate_id = debates.id 
        inner join plenar_infos on debates.plenar_id = plenar_infos.id 
        inner join delegates on delegates.id = plenar_speeches.delegate_id 
    where 
        plenar_infos.legislative_period = $1
        --and delegates.council = 'nr'
    group by 
        plenar_speeches.delegate_id, 
        delegates.image_url, 
        delegates.name, 
        delegates.party, 
        delegates.council 
    order by 
        hours_spoken DESC;",
    )
    .bind::<Text, _>(&legis_period.period)
    .load::<SpeakerByHours>(con)
}

#[cfg(test)]
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
        .filter(accepted.is_not_null())
        .load::<DbLegislativeInitiativeQuery>(con)
}

pub fn get_latest_legislative_initiatives(
    con: &mut PgConnection,
) -> QueryResult<Vec<DbLegislativeInitiativeQuery>> {
    legislative_initiatives
        .filter(created_at.gt(today() - chrono::Duration::days(30)))
        .filter(created_at.lt(today()))
        .filter(accepted.is_not_null())
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

pub fn get_parties(con: &mut PgConnection) -> QueryResult<Vec<DbParty>> {
    parties.load::<DbParty>(con)
}

#[cfg(test)]
mod tests {
    use crate::{
        dataservice::{
            dataservice_con, get_call_to_orders_per_party_delegates,
            get_delegates_by_call_to_orders, get_speakers_by_hours_by_legis_period,
        },
        routes::{LegisPeriod, RequestFilter},
        today,
    };

    use super::{
        get_delegates, get_delegates_by_call_to_orders_by_legis_period, get_latest_vote_results,
        get_legislative_initiatives, get_parties, get_speakers_by_hours,
    };

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

    #[test]
    fn test_get_speakers_by_hours() {
        let con = &mut dataservice_con();
        let res = get_speakers_by_hours(con).unwrap();

        println!("res: {res:?}");
    }

    #[test]
    fn test_get_speakers_by_hours_by_legis_period() {
        let con = &mut dataservice_con();
        let res = get_speakers_by_hours_by_legis_period(
            con,
            &LegisPeriod {
                period: "XXIII".into(),
            },
        )
        .unwrap();

        println!("res: {res:?}");
    }

    #[test]
    fn test_get_call_to_orders_by_delegates() {
        let con = &mut dataservice_con();
        let res = get_delegates_by_call_to_orders(con).unwrap();

        println!("res: {res:?}");
    }

    #[test]
    fn test_get_call_to_orders_per_party_delegates() {
        let con = &mut dataservice_con();
        let res = get_call_to_orders_per_party_delegates(con).unwrap();

        println!("res: {res:?}");
    }

    #[test]
    fn test_get_parties() {
        let con = &mut dataservice_con();
        let res = get_parties(con);
        println!("res: {res:?}");
    }

    #[test]
    fn test_get_delegates_by_call_to_orders_by_legis_period() {
        let con = &mut dataservice_con();
        let res = get_delegates_by_call_to_orders_by_legis_period(
            con,
            &LegisPeriod {
                period: "XXVII".into(),
            },
        )
        .unwrap();
        println!("res: {res:?}")
    }
}
