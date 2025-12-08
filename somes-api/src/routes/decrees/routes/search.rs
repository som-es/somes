use crate::{
    meilisearch::MeilisearchClient,
    routes::{DecreesWithMaxPage, FilterError},
    DECREES_PER_PAGE,
};
use axum::{extract::Query, Json};
use dataservice::combx::OptionalDecree;
use meilisearch_sdk::search::SearchResults;
use somes_common_lib::{DecreeFilter, Page};

pub async fn decrees_by_search_route(
    MeilisearchClient(meilisearch_client): MeilisearchClient,
    Query(search_query): Query<somes_common_lib::SearchQuery>,
    Query(page): Query<somes_common_lib::Page>,
    Json(decrees_filter): Json<Option<DecreeFilter>>,
) -> Result<Json<DecreesWithMaxPage>, FilterError> {
    meilisearch_decrees(
        meilisearch_client,
        search_query,
        page,
        decrees_filter.as_ref(),
    )
    .await
    .map(Json)
}

async fn meilisearch_decrees(
    meilisearch_client: meilisearch_sdk::client::Client,
    search_query: somes_common_lib::SearchQuery,
    page: Page,
    legis_init_filter: Option<&DecreeFilter>,
) -> Result<DecreesWithMaxPage, FilterError> {
    let stats = meilisearch_client
        .index("decrees")
        .get_stats()
        .await
        .unwrap();
    println!("{:?}", stats);
    let mut meilisearch_filter = String::new();
    if let Some(filter) = legis_init_filter.as_ref() {
        let mut filter_conditions = vec![];

        if let Some(ref legis_period) = filter.legis_period {
            filter_conditions.push(format!("gp = '{}'", legis_period));
        }
        if let Some(ref legis_period) = filter.gov_officials {
            filter_conditions.push({
                let ors = legis_period
                    .iter()
                    .map(|gov_official| format!("gov_official_id = {gov_official}"))
                    .collect::<Vec<_>>()
                    .join(" OR ");
                format!("({ors})")
            });
        }

        meilisearch_filter = filter_conditions.join(" AND ")
    }
    log::info!("decrees meilisearch filter: {meilisearch_filter}, {search_query:?}");

    let results: SearchResults<OptionalDecree> = meilisearch_client
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
        .map(|hit| hit.result)
        .collect::<Vec<_>>();

    Ok(DecreesWithMaxPage {
        decrees,
        entry_count: results.estimated_total_hits.unwrap_or(1) as i64,
        max_page,
    })
}
