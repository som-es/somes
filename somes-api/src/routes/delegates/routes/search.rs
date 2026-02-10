use axum::{extract::Query, Json};
use dataservice::combx::{meilisearch_filters_gov_props, Index};
use meilisearch_sdk::search::SearchResults;
use serde::{Deserialize, Serialize};
use somes_common_lib::{Delegate, DelegateFilter};
use somes_meilisearch_filter::{to_meilisearch_filters, FilterOptions};
use utoipa::ToSchema;

use crate::{meilisearch::MeilisearchClient, routes::FilterError, Qs, RedisConnection};

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct DelegatesWithMaxPage {
    pub delegates: Vec<Delegate>,
    pub entry_count: i64,
    pub max_page: i64,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

pub async fn delegates_by_search_route(
    RedisConnection(mut redis_con): RedisConnection,
    MeilisearchClient(meilisearch_client): MeilisearchClient,
    Query(search_query): Query<somes_common_lib::SearchQuery>,
    Query(page): Query<somes_common_lib::Page>,
    Query(entry_count_per_page): Query<somes_common_lib::PageEntryCount>,
    Query(sort): Query<somes_common_lib::SortParams>,
    Qs(delegate_filter): Qs<DelegateFilter>,
) -> Result<Json<DelegatesWithMaxPage>, FilterError> {
    let mut filter_conditions = to_meilisearch_filters(
        &delegate_filter.filter_arguments(),
        &FilterOptions::default(),
    );

    filter_conditions.extend(to_meilisearch_filters(
        &delegate_filter
            .mandates
            .unwrap_or_default()
            .iter()
            .map(|mandate| mandate.filter_arguments())
            .flatten()
            .collect::<Vec<_>>(),
        &FilterOptions {
            prefix: Some("mandates".into()),
            ..Default::default()
        },
    ));

    let meilisearch_filter = filter_conditions.join(" AND ");

    log::info!("meilisearch filter: {meilisearch_filter}");

    // match sort.sort.unwrap_or_default() {
    //     somes_common_lib::Sort::Asc => todo!(),
    //     somes_common_lib::Sort::Desc => todo!(),
    // }

    let results: SearchResults<Delegate> = meilisearch_client
        .index("delegates")
        .search()
        .with_filter(&meilisearch_filter)
        .with_query(&search_query.search.unwrap_or_default())
        .with_hits_per_page(entry_count_per_page.entries_per_page.unwrap_or(100) as usize)
        .with_page(page.page as usize)
        // .with_sort(&[""])
        .execute()
        .await?;

    let max_page = results.total_pages.unwrap_or(1) as i64;

    let updated_at =
        crate::meilisearch::get_update_time_of_index(&mut redis_con, &Index::Delegates)
            .await
            .ok()
            .map(|date| date.naive_local());

    let delegates = results
        .hits
        .into_iter()
        .map(|hit| hit.result)
        .collect::<Vec<_>>();
    Ok(Json(DelegatesWithMaxPage {
        delegates,
        entry_count: results.estimated_total_hits.unwrap_or(1) as i64,
        max_page,
        updated_at,
    }))
}
