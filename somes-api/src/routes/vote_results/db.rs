use chrono::NaiveDateTime;
use dataservice::{
    combx::{DbNamedVote, DbNamedVoteInfo, DbNamedVoteInfoQuery, OptionalVoteResult, Topic},
    db::models::{DbLegislativeInitiativeQuery, DbNamedVotes, DbSpeechWithLink, DbVote},
};
use redis::aio::MultiplexedConnection;
use serde::{Deserialize, Serialize};
use somes_common_lib::{AddonVoteResultFilter, Document};
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(ToSchema, Debug, Deserialize, Serialize, Clone)]
pub struct UniqueTopic {
    pub topic: String,
    pub id: i32,
}

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct VoteResultsWithMaxPage {
    pub vote_results: Vec<OptionalVoteResult>,
    pub entry_count: i64,
    pub max_page: i64,
    pub updated_at: Option<NaiveDateTime>,
}

use super::{
    construct_vote_result::construct_vote_result, filter::filtered_legislative_initiatives,
};

pub async fn filtered_legis_inits_per_page(
    pg: &PgPool,
    page: i64,
    page_elements: i64,
    filter: Option<&AddonVoteResultFilter>,
    is_finished: bool,
) -> sqlx::Result<(Vec<DbLegislativeInitiativeQuery>, i64)> {
    let res = match filter {
        Some(filter) => {
            filtered_legislative_initiatives(pg, page, page_elements, filter, is_finished).await?
        }
        None => {
            assert!(
                is_finished,
                "Add filter object when using not finished extraction"
            );
            (sqlx::query_as!(DbLegislativeInitiativeQuery,
                "select distinct * from legislative_initiatives where is_voteable_on and accepted is not null order by nr_plenary_activity_date desc offset $1 limit $2",
                page * page_elements,
                page_elements
            )
            .fetch_all(pg)
            .await?,

            sqlx::query!(
                "select distinct COUNT(*) from legislative_initiatives where accepted is not null",
            ).fetch_one(pg).await?.count.unwrap_or_default()
            )
        }
    };
    Ok(res)
}

pub async fn vote_result_by_unique_hints_with_accepted_required_sqlx(
    redis_con: MultiplexedConnection,
    pg: &PgPool,
    gp: &str,
    ityp: &str,
    inr: i32,
) -> sqlx::Result<Option<OptionalVoteResult>> {
    let legis_init = sqlx::query_as!(
        DbLegislativeInitiativeQuery,
        "select * from legislative_initiatives where gp = $1 and ityp = $2 and inr = $3 and accepted is not null",
        gp,
        ityp,
        inr
    )
    .fetch_optional(pg)
    .await?;
    if let Some(legis_init) = legis_init {
        construct_vote_result(redis_con, pg, legis_init.id).await
    } else {
        Ok(None)
    }
}

pub async fn vote_results_per_page_sqlx(
    redis_con: MultiplexedConnection,
    pg: &PgPool,
    page: i64,
    page_elements: i64,
    filter: Option<&AddonVoteResultFilter>,
    is_finished: bool,
) -> sqlx::Result<(Vec<OptionalVoteResult>, i64)> {
    let (entries, entry_count) =
        filtered_legis_inits_per_page(pg, page, page_elements, filter, is_finished).await?;

    let entries = entries
        .into_iter()
        .map(|legis_init| construct_vote_result(redis_con.clone(), pg, legis_init.id))
        .collect::<Vec<_>>();

    futures::future::join_all(entries)
        .await
        .into_iter()
        .collect::<sqlx::Result<Vec<_>>>()
        .map(|x| (x.into_iter().flatten().collect(), entry_count))
}

pub async fn eurovoc_topics_from_legis_init_sqlx(
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

pub async fn absences_delegate_ids_sqlx(
    con: &PgPool,
    plenary_session_id: i32,
) -> sqlx::Result<Vec<i32>> {
    sqlx::query!(
        "select delegate_id from absences where plenary_session_id = $1",
        plenary_session_id
    )
    .map(|x| x.delegate_id)
    .fetch_all(con)
    .await
}

pub async fn votes_from_legis_init_sqlx(
    con: &PgPool,
    legis_init_id: i32,
) -> sqlx::Result<Vec<DbVote>> {
    sqlx::query_as!(
        DbVote,
        "select party, code, votes.fraction, infavor from votes inner join parties on parties.name = votes.party where legislative_initiatives_id = $1",
        legis_init_id
    )
    .fetch_all(con)
    .await
}

pub async fn all_votes_from_legis_init(
    redis_con: MultiplexedConnection,
    con: &PgPool,
) -> sqlx::Result<Vec<OptionalVoteResult>> {
    let legis_inits = sqlx::query_as!(
        DbLegislativeInitiativeQuery,
        "SELECT DISTINCT * FROM legislative_initiatives where is_voteable_on"
    )
    .fetch_all(con)
    .await?;

    let mut vote_results = Vec::with_capacity(legis_inits.len());

    for legis_init in legis_inits {
        match construct_vote_result(redis_con.clone(), con, legis_init.id).await {
            Ok(vote_result) => {
                if let Some(vote_result) = vote_result {
                    vote_results.push(vote_result);
                }
            }
            Err(e) => {
                log::warn!("Error while constructing vote result, skipped in result of it: {e:?}")
            }
        }
    }

    Ok(vote_results)
}

pub async fn named_votes_from_legis_init_sqlx(
    con: &PgPool,
    legis_init_id: i32,
    is_named_vote: bool,
) -> sqlx::Result<Option<DbNamedVotes>> {
    if !is_named_vote {
        return Ok(None);
    }
    let named_vote_info = sqlx::query_as!(
        DbNamedVoteInfoQuery,
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
        "select id, infavor, was_absent, lev, similiarity_score, searched_with, matched_with, delegate_id, manually_matched from named_votes where named_vote_info_id = $1",
        named_vote_info.id
    )
    .fetch_all(con)
    .await?;

    Ok(Some(DbNamedVotes {
        named_vote_info: DbNamedVoteInfo {
            pro_count: named_vote_info.pro_count,
            contra_count: named_vote_info.contra_count,
            given_vote_sum: named_vote_info.given_vote_sum,
            invalid_count: named_vote_info.invalid_count,
        },
        named_votes: Some(named_votes),
    }))
}

pub async fn speeches_from_legis_init_sqlx(
    con: &PgPool,
    legis_init_id: i32,
) -> sqlx::Result<Vec<DbSpeechWithLink>> {
    sqlx::query_as!(
        DbSpeechWithLink,
        "select
            null as about, delegate_id, infavor, opinion, document_url, CAST(null AS int) as duration_in_seconds, legislative_initiatives_id as legis_init_id
        from
            speeches
        inner join
            speeches_html_urls on speeches.id = speeches_html_urls.speech_id

        where legislative_initiatives_id = $1

            ;",
        legis_init_id
    )
    .fetch_all(con)
    .await
}

pub async fn legis_docs_from_legis_init_sqlx(
    con: &PgPool,
    legis_init_id: i32,
) -> sqlx::Result<Vec<Document>> {
    sqlx::query_as!(
        Document,
        "select
            title, document_url, document_type
        from
            legislative_documents
         where legislative_initiatives_id = $1;",
        legis_init_id
    )
    .fetch_all(con)
    .await
}
