use common_scrapes::RelatedDelegate;
use dataservice::db::models::{
    DbLegisDocument, DbLegisDocumentOptional, DbLegislativeInitiativeQuery,
    DbMinistrialProposalQuery, DbNamedVote, DbNamedVoteInfo, DbNamedVotes, DbSpeech,
    DbSpeechWithLink, DbVote,
};
use redis::{aio::MultiplexedConnection, FromRedisValue};
use serde::{Deserialize, Serialize};
use somes_common_lib::LegisInitFilter;
use sqlx::{FromRow, PgPool};
use utoipa::ToSchema;

#[derive(ToSchema, Debug, Deserialize, Serialize, Clone)]
pub struct Topic {
    pub topic: String,
}

#[derive(ToSchema, Debug, Deserialize, Serialize, Clone)]
pub struct UniqueTopic {
    pub topic: String,
    pub id: i32,
}

#[derive(ToSchema, Debug, Deserialize, Serialize, Clone)]
pub struct GovProposal {
    pub ministrial_proposal: DbMinistrialProposalQuery,
    pub topics: Vec<Topic>,
    pub vote_result: Option<VoteResult>,
}

#[derive(ToSchema, Debug, Deserialize, Serialize, Clone, FromRow)]
pub struct VoteResult {
    pub id: i32,
    pub legislative_initiative: DbLegislativeInitiativeQuery,
    pub votes: Vec<DbVote>,
    pub speeches: Vec<DbSpeechWithLink>,
    pub named_votes: Option<DbNamedVotes>,
    pub topics: Vec<Topic>,
    pub documents: Vec<DbLegisDocumentOptional>,
    pub absences: Vec<i32>,
    pub issued_by_dels: Vec<RelatedDelegate>,
    pub referenced_by_others_ids: Vec<i32>,
    pub references: Vec<i32>,
}

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct VoteResultsWithMaxPage {
    pub vote_results: Vec<VoteResult>,
    pub entry_count: i64,
    pub max_page: i64,
}

use crate::{get_json_cache, set_json_cache, today};

use super::{
    construct_vote_result::construct_vote_result, filtering::filtered_legislative_initiatives,
    get_eurovoc_topics_from_ministrial_proposal,
};

pub async fn get_latest_legis_inits_per_page(
    pg: &PgPool,
    page: i64,
    page_elements: i64,
    filter: Option<&LegisInitFilter>,
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
                "select distinct * from legislative_initiatives where is_voteable_on and accepted is not null order by created_at desc offset $1 limit $2",
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

pub async fn get_latest_legislative_initiatives_sqlx(
    pg: &PgPool,
) -> sqlx::Result<Vec<DbLegislativeInitiativeQuery>> {
    let res = sqlx::query_as!(
        DbLegislativeInitiativeQuery,
        "select * from legislative_initiatives 
            where created_at = (select MAX(created_at) from legislative_initiatives 
            where accepted is not null) and accepted is not null and is_voteable_on"
    )
    .fetch_all(pg)
    .await?;
    Ok(res)
}

pub async fn get_latest_vote_results_sqlx(
    mut redis_con: MultiplexedConnection,
    pg: &PgPool,
) -> sqlx::Result<Vec<VoteResult>> {
    futures::future::join_all(
        get_latest_legislative_initiatives_sqlx(pg)
            .await?
            .into_iter()
            .map(|legis_init| construct_vote_result(redis_con.clone(), pg, legis_init))
            .collect::<Vec<_>>(),
    )
    .await
    .into_iter()
    .collect::<sqlx::Result<Vec<VoteResult>>>()
}

pub async fn get_vote_result_by_id(
    mut redis_con: MultiplexedConnection,
    pg: &PgPool,
    legis_init_id: i32,
) -> sqlx::Result<VoteResult> {
    let legis_init = sqlx::query_as!(
        DbLegislativeInitiativeQuery,
        "select * from legislative_initiatives where id = $1",
        legis_init_id
    )
    .fetch_one(pg)
    .await?;
    construct_vote_result(redis_con.clone(), pg, legis_init).await
}

pub async fn get_vote_result_by_path(
    mut redis_con: MultiplexedConnection,
    pg: &PgPool,
    gp: &str,
    ityp: &str,
    inr: i32,
) -> sqlx::Result<VoteResult> {
    let legis_init = sqlx::query_as!(
        DbLegislativeInitiativeQuery,
        "select * from legislative_initiatives where gp = $1 and ityp = $2 and inr = $3",
        gp,
        ityp,
        inr
    )
    .fetch_one(pg)
    .await?;
    construct_vote_result(redis_con.clone(), pg, legis_init).await
}

pub async fn construct_gov_proposal(
    mut redis_con: MultiplexedConnection,
    pg: &PgPool,
    ministrial_proposal: DbMinistrialProposalQuery,
) -> sqlx::Result<GovProposal> {
    let key = format!("ministrial_prop/{}", ministrial_proposal.id);
    let res = get_json_cache::<GovProposal>(&mut redis_con, &key).await;
    if let Some(res) = res {
        return Ok(res);
    }

    let vote_result = match (
        &ministrial_proposal.legis_init_gp,
        &ministrial_proposal.legis_init_ityp,
        ministrial_proposal.legis_init_inr,
    ) {
        (Some(ref gp), Some(ref ityp), Some(ref inr)) => {
            get_vote_result_by_unique_hints_with_accepted_required(
                redis_con.clone(),
                &pg,
                &gp,
                &ityp,
                *inr,
            )
            .await?
        }
        _ => None,
    };
    let gov_proposal = GovProposal {
        topics: get_eurovoc_topics_from_ministrial_proposal(pg, ministrial_proposal.id).await?,
        ministrial_proposal,
        vote_result,
    };

    let cache_date = if let Some(ref vote_result) = gov_proposal.vote_result {
        vote_result.legislative_initiative.created_at
    } else {
        today()
    };
    crate::set_json_cache_with_relevance(&mut redis_con, &key, &gov_proposal, cache_date)
        .await
        .ok_or(sqlx::Error::WorkerCrashed)?;

    Ok(gov_proposal)
}

pub async fn get_vote_result_by_unique_hints(
    redis_con: MultiplexedConnection,
    pg: &PgPool,
    gp: &str,
    ityp: &str,
    inr: i32,
) -> sqlx::Result<VoteResult> {
    let legis_init = sqlx::query_as!(
        DbLegislativeInitiativeQuery,
        "select * from legislative_initiatives where gp = $1 and ityp = $2 and inr = $3",
        gp,
        ityp,
        inr
    )
    .fetch_one(pg)
    .await?;

    let res = construct_vote_result(redis_con, pg, legis_init).await?;
    Ok(res)
}

pub async fn get_vote_result_by_unique_hints_with_accepted_required(
    redis_con: MultiplexedConnection,
    pg: &PgPool,
    gp: &str,
    ityp: &str,
    inr: i32,
) -> sqlx::Result<Option<VoteResult>> {
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
        construct_vote_result(redis_con, pg, legis_init)
            .await
            .map(|e| Some(e))
    } else {
        Ok(None)
    }
}

pub async fn get_latest_vote_results_sqlx_per_page(
    redis_con: MultiplexedConnection,
    pg: &PgPool,
    page: i64,
    page_elements: i64,
    filter: Option<&LegisInitFilter>,
    is_finished: bool,
) -> sqlx::Result<(Vec<VoteResult>, i64)> {
    let (entries, entry_count) =
        get_latest_legis_inits_per_page(pg, page, page_elements, filter, is_finished).await?;

    let entries = entries
        .into_iter()
        .map(|legis_init| construct_vote_result(redis_con.clone(), pg, legis_init))
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

pub async fn get_absences_delegate_ids_sqlx(
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

pub async fn get_all_votes_from_legis_init(
    redis_con: MultiplexedConnection,
    con: &PgPool,
) -> sqlx::Result<Vec<VoteResult>> {
    let legis_inits = sqlx::query_as!(
        DbLegislativeInitiativeQuery,
        "SELECT DISTINCT * FROM legislative_initiatives where is_voteable_on"
    )
    .fetch_all(con)
    .await?;

    let mut vote_results = Vec::with_capacity(legis_inits.len());

    for legis_init in legis_inits {
        match construct_vote_result(redis_con.clone(), con, legis_init).await {
            Ok(vote_result) => vote_results.push(vote_result),
            Err(e) => {
                log::warn!("Error while constructing vote result, skipped in result of it: {e:?}")
            }
        }
    }

    Ok(vote_results)
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
) -> sqlx::Result<Vec<DbSpeechWithLink>> {
    sqlx::query_as!(
        DbSpeechWithLink,
        "select 
            null as about, delegate_id, infavor, opinion, document_url, legislative_initiatives_id
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

pub async fn get_legis_docs_from_legis_init_sqlx(
    con: &PgPool,
    legis_init_id: i32,
) -> sqlx::Result<Vec<DbLegisDocumentOptional>> {
    sqlx::query_as!(
        DbLegisDocumentOptional,
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
