use axum::{Json, extract::Path};
use combx::with_data::{
    delegates::{extract_call_to_orders_by_delegate, extract_named_votes_by_delegate},
    unique_topics::EurovocTopics,
};
use common_scrapes::language::Language;
use somes_common_lib::GeneralDelegateInfo;
use sqlx::PgPool;

use crate::{
    Eurovoc, IS_PROD, PgPoolConnection, RedisConnection, get_json_cache,
    routes::{
        DelegateError::{self, SqlFailure},
        delegates::{
            left_right_topic_score::extract_political_position_by_delegate,
            stance_topic_score::extract_stance_topic_score_by_delegate,
        },
        extract_absences_by_delegate, extract_detailed_interests_of_delegate,
        extract_interests_of_delegate, extract_issued_proposals_by_delegate,
    },
};

pub async fn extended_delegate_info_route(
    PgPoolConnection(pg): PgPoolConnection,
    RedisConnection(mut redis_con): RedisConnection,
    Path(id): Path<i32>,
    Eurovoc(eurovoc_topics): Eurovoc,
) -> Result<Json<GeneralDelegateInfo>, DelegateError> {
    Ok(
        extract_general_delegate_info(id, &pg, &mut redis_con, &eurovoc_topics)
            .await
            .map(Json)?,
    )
}

pub async fn extract_general_delegate_info(
    delegate_id: i32,
    pg: &PgPool,
    redis_con: &mut (impl redis::aio::ConnectionLike + Send + Sync),
    eurovoc_topics: &EurovocTopics,
) -> Result<GeneralDelegateInfo, DelegateError> {
    let key = format!("general_delegate_info_{delegate_id}");

    if *IS_PROD {
        let res = get_json_cache::<GeneralDelegateInfo>(redis_con, &key).await;
        if let Some(res) = res {
            return Ok(res);
        }
    }

    let start = tokio::time::Instant::now();
    let interests = extract_interests_of_delegate(delegate_id, pg)
        .await
        .map_err(|e| SqlFailure(e))?;
    println!("interests took {:?}", start.elapsed());
    let detailed_interests = extract_detailed_interests_of_delegate(delegate_id, pg)
        .await
        .map_err(|e| SqlFailure(e))?;

    println!("detailed_interests took {:?}", start.elapsed());
    // let delegate_qa = extract_delegate_qa(delegate_id, pg).await?;
    let delegate_qa = vec![];
    println!("delegate_qa took {:?}", start.elapsed());
    let absences = extract_absences_by_delegate(pg, delegate_id)
        .await
        .map_err(|e| SqlFailure(e))?;

    println!("absences took {:?}", start.elapsed());
    let named_votes = extract_named_votes_by_delegate(pg, delegate_id)
        .await
        .map_err(|e| SqlFailure(e))?;

    println!("named_votes took {:?}", start.elapsed());
    let political_position =
        extract_political_position_by_delegate(pg, delegate_id, eurovoc_topics, Language::De)
            .await?;

    println!("left_right_stances took {:?}", start.elapsed());
    let (stance_topic_influences, stance_topic_scores) =
        extract_stance_topic_score_by_delegate(pg, delegate_id)
            .await
            .map_err(|e| SqlFailure(e))?;

    println!("stance_topic_influences took {:?}", start.elapsed());
    let received_call_to_orders = extract_call_to_orders_by_delegate(delegate_id, pg)
        .await
        .map_err(|e| SqlFailure(e))?;

    println!("received_call_to_orders took {:?}", start.elapsed());
    let issued_proposals = extract_issued_proposals_by_delegate(delegate_id, pg)
        .await
        .map_err(|e| SqlFailure(e))?;

    println!("issued_proposals took {:?}", start.elapsed());

    let gdi = GeneralDelegateInfo {
        interests,
        detailed_interests,
        delegate_qa,
        absences,
        named_votes,
        political_position,
        stance_topic_influences,
        stance_topic_scores,
        received_call_to_orders,
        issued_proposals,
    };

    let _ = crate::set_json_cache(redis_con, &key, &gdi).await;
    Ok(gdi)
}
