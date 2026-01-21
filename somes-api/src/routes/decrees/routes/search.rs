use crate::{
    meilisearch::MeilisearchClient,
    routes::{DecreesWithMaxPage, FilterError},
    Qs, RedisConnection, DECREES_PER_PAGE,
};
use axum::{extract::Query, Json};
use chrono::DateTime;
use dataservice::combx::OptionalDecreeFilter;
use meilisearch_sdk::search::SearchResults;
use serde_json::Value;
use somes_common_lib::Page;
use somes_meilisearch_filter::to_meilisearch_filter;

pub async fn decrees_by_search_route(
    RedisConnection(mut redis_con): RedisConnection,
    MeilisearchClient(meilisearch_client): MeilisearchClient,
    Query(search_query): Query<somes_common_lib::SearchQuery>,
    Query(page): Query<somes_common_lib::Page>,
    Qs(decrees_filter): Qs<OptionalDecreeFilter>,
) -> Result<Json<DecreesWithMaxPage>, FilterError> {
    meilisearch_decrees(
        &mut redis_con,
        meilisearch_client,
        search_query,
        page,
        decrees_filter,
    )
    .await
    .map(Json)
}

async fn meilisearch_decrees(
    redis_con: &mut redis::aio::MultiplexedConnection,
    meilisearch_client: meilisearch_sdk::client::Client,
    search_query: somes_common_lib::SearchQuery,
    page: Page,
    decree_filter: OptionalDecreeFilter,
) -> Result<DecreesWithMaxPage, FilterError> {
    let meilisearch_filter = to_meilisearch_filter(&decree_filter.filter_arguments());

    // let stats = meilisearch_client
    //     .index("decrees")
    //     .get_stats()
    //     .await
    //     .unwrap();
    // println!("{:?}", stats);

    log::info!("decrees meilisearch filter: {meilisearch_filter}, {search_query:?}");

    let results: SearchResults<Value> = meilisearch_client
        .index("decrees")
        .search()
        .with_filter(&meilisearch_filter)
        .with_sort(&["publication_date:desc"])
        .with_query(&search_query.search)
        .with_hits_per_page(DECREES_PER_PAGE.parse().unwrap_or(16))
        .with_page(page.page as usize)
        .execute()
        .await?;

    let max_page = results.total_pages.unwrap_or(1) as i64;

    let decrees = results
        .hits
        .into_iter()
        .map(|hit| {
            let mut decree = hit.result;
            if let Some(timestamp) = decree["publication_date"].as_i64() {
                let date = DateTime::from_timestamp(timestamp, 0).map(|dt| dt.date_naive());
                decree["publication_date"] = serde_json::json!(date);
            }
            if let Some(timestamp) = decree["created_at"].as_i64() {
                let date = DateTime::from_timestamp(timestamp, 0);
                decree["created_at"] = serde_json::json!(date);
            }
            if let Some(timestamp) = decree["updated_at"].as_i64() {
                let date = DateTime::from_timestamp(timestamp, 0);
                decree["updated_at"] = serde_json::json!(date);
            }

            serde_json::from_value(decree).expect("Failed to deserialize Decree")
        })
        .collect::<Vec<_>>();

    let updated_at = crate::meilisearch::get_update_time_of_index(
        redis_con,
        &crate::meilisearch::Index::Decrees,
    )
    .await
    .ok()
    .map(|date| date.naive_local());

    Ok(DecreesWithMaxPage {
        decrees,
        entry_count: results.estimated_total_hits.unwrap_or(1) as i64,
        max_page,
        updated_at,
    })
}
