use dataservice::{combx::OptionalVoteResult, db::models::*};
use redis::aio::MultiplexedConnection;
use somes_common_lib::Document;
use sqlx::PgPool;

use crate::get_json_cache;

pub async fn fetch_all_vote_results(pg: &PgPool) -> sqlx::Result<Vec<OptionalVoteResult>> {
    sqlx::query_as!(OptionalVoteResult, "select * from vote_results")
        .fetch_all(pg)
        .await
}

pub async fn fetch_vote_result_by_id(pg: &PgPool, id: i32) -> sqlx::Result<OptionalVoteResult> {
    sqlx::query_as!(
        OptionalVoteResult,
        "select * from vote_results where id = $1",
        id
    )
    .fetch_one(pg)
    .await
}

#[tokio::test]
async fn test_fetch_all_vote_results() {
    let pg = dataservice::combx::connect_pg().await;
    println!("start fetch...");
    let start = tokio::time::Instant::now();
    let legis_inits = sqlx::query_as!(
        DbLegislativeInitiativeQuery,
        "SELECT DISTINCT * FROM legislative_initiatives where is_voteable_on"
    )
    .fetch_all(&pg)
    .await
    .unwrap();

    let client = redis::Client::open(crate::REDIS_DB).unwrap();
    let redis_con = client.get_multiplexed_async_connection().await.unwrap();
    for legis_init in legis_inits {
        dbg!(legis_init.id);

        let start = std::time::Instant::now();
        let mut lhs = fetch_vote_result_by_id(&pg, legis_init.id).await.unwrap();
        lhs.meilisearch_helper = Some(MeilisearchHelper { votes: vec![] });
        println!("elapsed (new): {:?}", start.elapsed());
    }
    // let vote_results = fetch_all_vote_results(&pg).await.unwrap();
    println!("elapsed: {:?}", start.elapsed());
    // dbg!(vote_results.len());
    // dbg!(vote_results.first());
}

pub async fn construct_vote_result(
    mut redis_con: MultiplexedConnection,
    pg: &PgPool,
    legis_init_id: i32,
) -> sqlx::Result<OptionalVoteResult> {
    let key = format!("vote_result/{}", legis_init_id.to_string());
    let res = get_json_cache::<OptionalVoteResult>(&mut redis_con, &key).await;
    if let Some(res) = res {
        return Ok(res);
    }

    let out = fetch_vote_result_by_id(pg, legis_init_id).await?;

    crate::set_json_cache_with_relevance(
        &mut redis_con,
        &key,
        &out,
        out.legislative_initiative.as_ref().unwrap().created_at,
    )
    .await
    .ok_or(sqlx::Error::WorkerCrashed)?;
    Ok(out)
}
