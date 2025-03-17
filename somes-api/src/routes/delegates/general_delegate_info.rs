use axum::{extract::Query, Json};
use redis::aio::MultiplexedConnection;
use somes_common_lib::{DelegateById, GeneralDelegateInfo, Mandate};
use sqlx::{query_as, PgPool};

use crate::{get_json_cache, PgPoolConnection, RedisConnection};

use super::{
    extract_absences_by_delegate, extract_delegate_qa, extract_detailed_interests_of_delegate,
    extract_interests_of_delegate, extract_political_position,
    named_votes::extract_named_votes_by_delegate,
    stance_topic_score::extract_stance_topic_score_by_delegate, DelegatesErrorResponse,
};

pub async fn extract_general_delegate_info(
    delegate_id: i32,
    pg: &PgPool,
    redis_con: &mut MultiplexedConnection,
) -> sqlx::Result<GeneralDelegateInfo> {
    let key = format!("general_delegate_info_{delegate_id}");

    let res = get_json_cache::<GeneralDelegateInfo>(redis_con, &key).await;
    if let Some(res) = res {
        return Ok(res);
    }

    let mandates = query_as!(
        Mandate,
        "
            select start_date, end_date, name from mandates where delegate_id = $1
        ",
        delegate_id
    )
    .fetch_all(pg)
    .await?;

    let interests = extract_interests_of_delegate(delegate_id, pg).await?;
    let detailed_interests = extract_detailed_interests_of_delegate(delegate_id, pg).await?;
    let delegate_qa = extract_delegate_qa(delegate_id, pg).await?;
    let political_position = extract_political_position(delegate_id, pg).await?;
    let absences = extract_absences_by_delegate(pg, delegate_id).await?;
    let named_votes = extract_named_votes_by_delegate(pg, delegate_id).await?;
    let stances = extract_stance_topic_score_by_delegate(pg, delegate_id).await?;

    let gdi = GeneralDelegateInfo {
        mandates,
        interests,
        detailed_interests,
        delegate_qa,
        political_position,
        absences,
        named_votes,
        stances,
    };

    crate::set_json_cache(redis_con, &key, &gdi)
        .await
        .ok_or(sqlx::Error::WorkerCrashed)?;
    Ok(gdi)
}

pub async fn general_delegate_info(
    PgPoolConnection(pg): PgPoolConnection,
    RedisConnection(mut redis_con): RedisConnection,
    Query(delegate_by_id): Query<DelegateById>,
) -> Result<Json<GeneralDelegateInfo>, DelegatesErrorResponse> {
    extract_general_delegate_info(delegate_by_id.delegate_id, &pg, &mut redis_con)
        .await
        .map(Json)
        .map_err(|e| DelegatesErrorResponse::DbSelectFailure(Some(e)))
}
