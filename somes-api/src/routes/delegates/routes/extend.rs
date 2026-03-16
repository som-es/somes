use axum::{extract::Path, Json};
use redis::aio::MultiplexedConnection;
use somes_common_lib::{GeneralDelegateInfo, Mandate};
use sqlx::{query_as, PgPool};

use crate::{
    PgPoolConnection, RedisConnection, get_json_cache, routes::{
        DelegateError, delegates::{
            left_right_topic_score::extract_left_right_topic_score_by_delegate,
            named_votes::extract_named_votes_by_delegate,
            stance_topic_score::extract_stance_topic_score_by_delegate,
        }, extract_absences_by_delegate, extract_call_to_orders_by_delegate, extract_delegate_qa, extract_detailed_interests_of_delegate, extract_interests_of_delegate, extract_issued_proposals_by_delegate, extract_political_position
    }
};

pub async fn extended_delegate_info_route(
    PgPoolConnection(pg): PgPoolConnection,
    RedisConnection(mut redis_con): RedisConnection,
    Path(id): Path<i32>,
) -> Result<Json<GeneralDelegateInfo>, DelegateError> {
    Ok(extract_general_delegate_info(id, &pg, &mut redis_con)
        .await
        .map(Json)?)
}

pub async fn extract_general_delegate_info(
    delegate_id: i32,
    pg: &PgPool,
    redis_con: &mut MultiplexedConnection,
) -> sqlx::Result<GeneralDelegateInfo> {
    let key = format!("general_delegate_info_{delegate_id}");

    // let res = get_json_cache::<GeneralDelegateInfo>(redis_con, &key).await;
    // if let Some(res) = res {
    //     return Ok(res);
    // }

    let start = tokio::time::Instant::now();
    let interests = extract_interests_of_delegate(delegate_id, pg).await?;
    println!("interests took {:?}", start.elapsed());
    let detailed_interests = extract_detailed_interests_of_delegate(delegate_id, pg).await?;
    println!("detailed_interests took {:?}", start.elapsed());
    let delegate_qa = extract_delegate_qa(delegate_id, pg).await?;
    println!("delegate_qa took {:?}", start.elapsed());
    let political_position = extract_political_position(delegate_id, pg).await?;
    println!("political_position took {:?}", start.elapsed());
    let absences = extract_absences_by_delegate(pg, delegate_id).await?;
    println!("absences took {:?}", start.elapsed());
    let named_votes = extract_named_votes_by_delegate(pg, delegate_id).await?;
    println!("named_votes took {:?}", start.elapsed());
    let left_right_stances = extract_left_right_topic_score_by_delegate(pg, delegate_id).await?;
    println!("left_right_stances took {:?}", start.elapsed());
    let (stance_topic_influences, stance_topic_scores) =
        extract_stance_topic_score_by_delegate(pg, delegate_id).await?;
        println!("stance_topic_influences took {:?}", start.elapsed());
    let received_call_to_orders = extract_call_to_orders_by_delegate(delegate_id, pg).await?;
    println!("received_call_to_orders took {:?}", start.elapsed());
    let issued_proposals = extract_issued_proposals_by_delegate(delegate_id, pg).await?;
    println!("issued_proposals took {:?}", start.elapsed());

    let gdi = GeneralDelegateInfo {
        interests,
        detailed_interests,
        delegate_qa,
        political_position,
        absences,
        named_votes,
        left_right_stances,
        stance_topic_influences,
        stance_topic_scores,
        received_call_to_orders,
        issued_proposals,
    };

    crate::set_json_cache(redis_con, &key, &gdi)
        .await
        .ok_or(sqlx::Error::WorkerCrashed)?;
    Ok(gdi)
}
