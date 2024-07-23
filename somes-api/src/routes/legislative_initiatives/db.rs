use dataservice::db::{
    models::{DbLegislativeInitiativeQuery, DbSpeech, DbVote},
    schema::{
        legislative_initiatives::{accepted, created_at, dsl::legislative_initiatives},
        votes::{dsl::votes, legislative_initiatives_id},
    },
};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};
use somes_common_lib::DateRange;
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::{dataservice::get_speeches_from_legis_init, today};

pub fn get_legislative_initiatives(
    con: &mut PgConnection,
    filter: DateRange,
) -> QueryResult<Vec<DbLegislativeInitiativeQuery>> {
    legislative_initiatives
        .filter(created_at.gt(filter.start))
        .filter(created_at.lt(filter.end))
        .filter(accepted.is_not_null())
        .load::<DbLegislativeInitiativeQuery>(con)
}

pub async fn get_latest_legis_inits_per_page(
    pg: &PgPool,
    page: i64,
    page_elements: i64,
) -> sqlx::Result<Vec<DbLegislativeInitiativeQuery>> {
    let res = sqlx::query!(
        "select * from legislative_initiatives where accepted is not null order by created_at desc offset $1 limit $2",
        page * page_elements,
        page_elements
    )
    .fetch_all(pg)
    .await?;
    Ok(res
        .into_iter()
        .map(|rec| DbLegislativeInitiativeQuery {
            id: rec.id,
            ityp: rec.ityp,
            gp: rec.gp,
            inr: rec.inr,
            emphasis: rec.emphasis,
            title: rec.title,
            description: rec.description,
            accepted: rec.accepted,
            created_at: rec.created_at,
            appeared_at: rec.appeared_at.map(|x| x.naive_local()),
            updated_at: rec.updated_at.map(|x| x.naive_local()),
            requires_simple_majority: rec.requires_simple_majority,
            was_invisibly_declined: rec.was_invisibly_declined,
        })
        .collect())
}

pub async fn get_latest_legislative_initiatives_sqlx(
    pg: &PgPool,
) -> sqlx::Result<Vec<DbLegislativeInitiativeQuery>> {
    let res = sqlx::query!(
        "select * from legislative_initiatives 
            where created_at = (select MAX(created_at) from legislative_initiatives 
            where accepted is not null) and accepted is not null"
    )
    .fetch_all(pg)
    .await?;
    Ok(res
        .into_iter()
        .map(|rec| DbLegislativeInitiativeQuery {
            id: rec.id,
            ityp: rec.ityp,
            gp: rec.gp,
            inr: rec.inr,
            emphasis: rec.emphasis,
            title: rec.title,
            description: rec.description,
            accepted: rec.accepted,
            created_at: rec.created_at,
            appeared_at: rec.appeared_at.map(|x| x.naive_local()),
            updated_at: rec.updated_at.map(|x| x.naive_local()),
            requires_simple_majority: rec.requires_simple_majority,
            was_invisibly_declined: rec.was_invisibly_declined,
        })
        .collect())
}

pub fn get_latest_legislative_initiatives(
    con: &mut PgConnection,
) -> QueryResult<Vec<DbLegislativeInitiativeQuery>> {
    legislative_initiatives
        // make the date range configurable as user !
        .filter(created_at.gt(today() - chrono::Duration::days(90)))
        .filter(created_at.lt(today()))
        .filter(accepted.is_not_null())
        .order(created_at.desc())
        .load::<DbLegislativeInitiativeQuery>(con)
}

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct Topic {
    pub topic: String
}

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct VoteResult {
    pub legislative_initiative: DbLegislativeInitiativeQuery,
    pub votes: Vec<DbVote>,
    pub speeches: Vec<DbSpeech>,
    pub topics: Vec<Topic>
}

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct VoteResultsWithMaxPage {
    pub vote_results: Vec<VoteResult>,
    pub max_page: i64,
}

pub async fn get_latest_vote_results_sqlx(pg: &PgPool) -> sqlx::Result<Vec<VoteResult>> {
    futures::future::join_all(
        get_latest_legislative_initiatives_sqlx(pg)
            .await?
            .into_iter()
            .map(|legis_init| async {
                Ok(VoteResult {
                    votes: get_votes_from_legis_init_sqlx(pg, legis_init.id).await?,
                    speeches: get_speeches_from_legis_init_sqlx(pg, legis_init.id).await?,
                    topics: get_eurovoc_topics_from_legis_init(pg, legis_init.id).await?,
                    legislative_initiative: legis_init,
                })
            })
            .collect::<Vec<_>>(),
    )
    .await
    .into_iter()
    .collect::<sqlx::Result<Vec<VoteResult>>>()
}

pub async fn get_latest_vote_results_sqlx_per_page(
    pg: &PgPool,
    page: i64,
    page_elements: i64,
) -> sqlx::Result<Vec<VoteResult>> {
    futures::future::join_all(
        get_latest_legis_inits_per_page(pg, page, page_elements)
            .await?
            .into_iter()
            .map(|legis_init| async {
                Ok(VoteResult {
                    votes: get_votes_from_legis_init_sqlx(pg, legis_init.id).await?,
                    speeches: get_speeches_from_legis_init_sqlx(pg, legis_init.id).await?,
                    topics: get_eurovoc_topics_from_legis_init(pg, legis_init.id).await?,
                    legislative_initiative: legis_init,
                })
            })
            .collect::<Vec<_>>(),
    )
    .await
    .into_iter()
    .collect::<sqlx::Result<Vec<VoteResult>>>()
}

pub fn get_latest_vote_results(con: &mut PgConnection) -> QueryResult<Vec<VoteResult>> {
    get_latest_legislative_initiatives(con)?
        .into_iter()
        .map(|legis_init| {
            Ok(VoteResult {
                votes: get_votes_from_legis_init(con, legis_init.id)?,
                speeches: get_speeches_from_legis_init(con, legis_init.id)?,
                topics: vec![],
                legislative_initiative: legis_init,
            })
        })
        .collect::<QueryResult<Vec<VoteResult>>>()
}

pub async fn get_eurovoc_topics_from_legis_init(
    con: &PgPool,
    legis_init_id: i32,
) -> sqlx::Result<Vec<Topic>> {
    sqlx::query_as!(
        Topic,
        "select topic from eurovoc_topics_legis_init where legislative_initiatives_id = $1",
        legis_init_id
    )
    .fetch_all(con)
    .await
}

pub async fn get_votes_from_legis_init_sqlx(
    con: &PgPool,
    legis_init_id: i32,
) -> sqlx::Result<Vec<DbVote>> {
    sqlx::query_as!(
        DbVote,
        "select * from votes where legislative_initiatives_id = $1",
        legis_init_id
    )
    .fetch_all(con)
    .await
}

pub fn get_votes_from_legis_init(
    con: &mut PgConnection,
    legis_init_id: i32,
) -> QueryResult<Vec<DbVote>> {
    votes
        .filter(legislative_initiatives_id.eq(legis_init_id))
        .load::<DbVote>(con)
}

pub async fn get_speeches_from_legis_init_sqlx(
    con: &PgPool,
    legis_init_id: i32,
) -> sqlx::Result<Vec<DbSpeech>> {
    sqlx::query_as!(
        DbSpeech,
        "select * from speeches where legislative_initiatives_id = $1",
        legis_init_id
    )
    .fetch_all(con)
    .await
}
