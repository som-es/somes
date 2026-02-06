use crate::{
    meilisearch::MeilisearchClient,
    routes::{DecreeDelegate, DecreeDelegateFilter, DecreesWithMaxPage, FilterError},
    Qs, RedisConnection, DECREES_PER_PAGE,
};
use axum::{extract::Query, Json};
use meilisearch_sdk::search::SearchResults;
use somes_common_lib::{Page, Sort};
use somes_meilisearch_filter::{to_meilisearch_filters, FilterOptions};

pub async fn decrees_by_search_route(
    RedisConnection(mut redis_con): RedisConnection,
    MeilisearchClient(meilisearch_client): MeilisearchClient,
    Query(search_query): Query<somes_common_lib::SearchQuery>,
    Query(page): Query<somes_common_lib::Page>,
    Query(entry_count_per_page): Query<somes_common_lib::PageEntryCount>,
    Query(sort): Query<somes_common_lib::SortParams>,
    Qs(decrees_filter): Qs<DecreeDelegateFilter>,
) -> Result<Json<DecreesWithMaxPage>, FilterError> {
    meilisearch_decrees(
        &mut redis_con,
        meilisearch_client,
        search_query,
        entry_count_per_page
            .entries_per_page
            .unwrap_or(DECREES_PER_PAGE.parse().unwrap_or(16)),
        sort.sort.unwrap_or_default(),
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
    entries_per_page: usize,
    sort: Sort,
    page: Page,
    decree_filter: DecreeDelegateFilter,
) -> Result<DecreesWithMaxPage, FilterError> {
    let mut filter_conditions =
        to_meilisearch_filters(&decree_filter.filter_arguments(), &FilterOptions::default());

    if let Some(decree_filter) = decree_filter.decree {
        filter_conditions.extend(to_meilisearch_filters(
            &decree_filter.filter_arguments(),
            &FilterOptions {
                prefix: Some("decree".into()),
                ..Default::default()
            },
        ));
    }
    if let Some(delegate) = &decree_filter.delegate {
        filter_conditions.extend(to_meilisearch_filters(
            &delegate.filter_arguments(),
            &FilterOptions {
                prefix: Some("delegate".into()),
                ..Default::default()
            },
        ));
    }
    let meilisearch_filter = filter_conditions.join(" AND ");

    // let stats = meilisearch_client
    //     .index("decrees")
    //     .get_stats()
    //     .await
    //     .unwrap();
    // println!("{:?}", stats);

    log::info!("decrees meilisearch filter: {meilisearch_filter}, {search_query:?}");

    let sort = match sort {
        Sort::Asc => "decree.publication_date:asc",
        Sort::Desc => "decree.publication_date:desc",
    };

    let results: SearchResults<DecreeDelegate> = meilisearch_client
        .index("decrees")
        .search()
        .with_filter(&meilisearch_filter)
        .with_sort(&[sort])
        .with_query(&search_query.search.unwrap_or_default())
        .with_hits_per_page(entries_per_page)
        .with_page(page.page as usize)
        .execute()
        .await?;

    let max_page = results.total_pages.unwrap_or(1) as i64;

    let decrees = results
        .hits
        .into_iter()
        .map(|decree| decree.result)
        .collect();

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
