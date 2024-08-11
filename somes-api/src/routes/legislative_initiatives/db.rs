use dataservice::db::models::{
    DbLegislativeInitiativeQuery, DbNamedVote, DbNamedVoteInfo, DbNamedVotes, DbSpeech, DbVote,
};
use diesel::{PgConnection, QueryResult};
use serde::{Deserialize, Serialize};
use somes_common_lib::LegisInitFilter;
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct Topic {
    pub topic: String,
}

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct VoteResult {
    pub legislative_initiative: DbLegislativeInitiativeQuery,
    pub votes: Vec<DbVote>,
    pub speeches: Vec<DbSpeech>,
    pub named_votes: Option<DbNamedVotes>,
    pub topics: Vec<Topic>,
}

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct VoteResultsWithMaxPage {
    pub vote_results: Vec<VoteResult>,
    pub entry_count: i64,
    pub max_page: i64,
}

use super::filtering::filtered_legislative_initiatives;

pub async fn get_latest_legis_inits_per_page(
    pg: &PgPool,
    page: i64,
    page_elements: i64,
    filter: Option<&LegisInitFilter>,
) -> sqlx::Result<(Vec<DbLegislativeInitiativeQuery>, i64)> {
    let res = match filter {
        Some(filter) => {
            filtered_legislative_initiatives(pg, page, page_elements, filter).await?
        }
        None => {
            (sqlx::query_as!(DbLegislativeInitiativeQuery,
                "select distinct * from legislative_initiatives where accepted is not null order by created_at desc offset $1 limit $2",
                page * page_elements,
                page_elements
            )
            .fetch_all(pg)
            .await?,

            sqlx::query!(
                "select distinct COUNT(*) from legislative_initiatives where accepted is not null",
            ).fetch_one(pg).await?.count.unwrap_or_default()
            )
        },
    };
    Ok(res)
}

pub async fn get_latest_legislative_initiatives_sqlx(
    pg: &PgPool,
) -> sqlx::Result<Vec<DbLegislativeInitiativeQuery>> {
    let res = sqlx::query_as!(
        DbLegislativeInitiativeQuery,
        "select * from legislative_initiatives 
            where created_at = (select MAX(created_at) from legislative_initiatives 
            where accepted is not null) and accepted is not null"
    )
    .fetch_all(pg)
    .await?;
    Ok(res)
}

pub async fn construct_vote_result(
    pg: &PgPool,
    legis_init: DbLegislativeInitiativeQuery,
) -> sqlx::Result<VoteResult> {
    let named_votes =
        get_named_votes_from_legis_init_sqlx(pg, legis_init.id, legis_init.voted_by_name).await?;
    let votes = if named_votes.is_some() {
        dataservice::with_data::named_votes::named_vote_pro_count_by_party(
            pg,
            legis_init.id,
            legis_init.created_at,
        )
        .await?
        .into_iter()
        .map(|vote| DbVote {
            party: vote.party.unwrap(),
            fraction: vote.pro_count.unwrap() as i32,
            infavor: true,
            legislative_initiatives_id: legis_init.id,
        })
        .collect()
    } else {
        get_votes_from_legis_init_sqlx(pg, legis_init.id).await?
    };

    Ok(VoteResult {
        votes,
        named_votes,
        speeches: get_speeches_from_legis_init_sqlx(pg, legis_init.id).await?,
        topics: get_eurovoc_topics_from_legis_init(pg, legis_init.id).await?,
        legislative_initiative: legis_init,
    })
}

pub async fn get_latest_vote_results_sqlx(pg: &PgPool) -> sqlx::Result<Vec<VoteResult>> {
    futures::future::join_all(
        get_latest_legislative_initiatives_sqlx(pg)
            .await?
            .into_iter()
            .map(|legis_init| construct_vote_result(pg, legis_init))
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
    filter: Option<&LegisInitFilter>,
) -> sqlx::Result<(Vec<VoteResult>, i64)> {
    let (entries, entry_count) =
        get_latest_legis_inits_per_page(pg, page, page_elements, filter).await?;

    let entries = entries
        .into_iter()
        .map(|legis_init| construct_vote_result(pg, legis_init))
        .collect::<Vec<_>>();

    futures::future::join_all(entries)
        .await
        .into_iter()
        .collect::<sqlx::Result<Vec<VoteResult>>>()
        .map(|x| (x, entry_count))
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

pub async fn get_named_votes_from_legis_init_sqlx(
    con: &PgPool,
    legis_init_id: i32,
    is_named_vote: bool,
) -> sqlx::Result<Option<DbNamedVotes>> {
    if !is_named_vote {
        return Ok(None);
    }
    let named_vote_info = sqlx::query_as!(
        DbNamedVoteInfo,
        "select * from named_vote_info where legis_init_id = $1",
        legis_init_id
    )
    .fetch_optional(con)
    .await?;

    let Some(named_vote_info) = named_vote_info else {
        return Ok(None);
    };

    let named_votes = sqlx::query_as!(
        DbNamedVote,
        "select * from named_votes where named_vote_info_id = $1",
        named_vote_info.id
    )
    .fetch_all(con)
    .await?;

    Ok(Some(DbNamedVotes {
        named_vote_info,
        named_votes,
    }))
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
