use crate::{
    DECREES_PER_PAGE, ParliamentCtx, Qs, RedisConnection,
    meilisearch::MeilisearchClient,
    routes::{DecreeDelegate, DecreeDelegateFilter, DecreesWithMaxPage, FilterError},
};
use axum::{Json, extract::Query};
use combx::{Index, meilisearch_filters_ai_summary};
use meilisearch_sdk::search::SearchResults;
use somes_common_lib::{Page, Sort, TopicsFilter};
use somes_meilisearch_filter::{FilterOptions, to_meilisearch_filters};

pub async fn decrees_by_search_route(
    ParliamentCtx(parliament): ParliamentCtx,
    RedisConnection(mut redis_con): RedisConnection,
    MeilisearchClient(meilisearch_client): MeilisearchClient,
    Query(search_query): Query<somes_common_lib::SearchQuery>,
    Query(page): Query<somes_common_lib::Page>,
    Query(entry_count_per_page): Query<somes_common_lib::PageEntryCount>,
    Query(sort): Query<somes_common_lib::SortParams>,
    Query(date_range): Query<somes_common_lib::DateRangeQueryFilter>,
    Query(topics): Query<TopicsFilter>,
    Qs(decrees_filter): Qs<DecreeDelegateFilter>,
) -> Result<Json<DecreesWithMaxPage>, FilterError> {
    meilisearch_decrees(
        parliament,
        &mut redis_con,
        meilisearch_client,
        search_query,
        entry_count_per_page
            .entries_per_page
            .unwrap_or(DECREES_PER_PAGE.parse().unwrap_or(16)),
        sort.sort,
        page,
        decrees_filter,
        date_range,
        topics,
    )
    .await
    .map(Json)
}

async fn meilisearch_decrees(
    parliament: combx::Parliament,
    redis_con: &mut redis::aio::MultiplexedConnection,
    meilisearch_client: meilisearch_sdk::client::Client,
    search_query: somes_common_lib::SearchQuery,
    entries_per_page: usize,
    sort: Option<Sort>,
    page: Page,
    decree_filter: DecreeDelegateFilter,
    date_range: somes_common_lib::DateRangeQueryFilter,
    topics: TopicsFilter,
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
        filter_conditions.extend(meilisearch_filters_ai_summary(
            decree_filter.ai_summary,
            Some("decree.ai_summary".into()),
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

    if let Some(date_from) = date_range.date_from {
        filter_conditions.push(format!(
            "decree.publication_date >= {:?}",
            date_from.to_string()
        ));
    }
    if let Some(date_to) = date_range.date_to {
        filter_conditions.push(format!(
            "decree.publication_date <= {:?}",
            date_to.to_string()
        ));
    }

    if let Some(topics) = topics.topics
        && !topics.is_empty()
    {
        let ai_summary_values = topics
            .iter()
            .map(|topic| format!("{topic:?}"))
            .collect::<Vec<_>>()
            .join(", ");

        filter_conditions.push(format!(
            "decree.ai_summary.full_summary.topics IN [{ai_summary_values}]"
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
        Some(Sort::Asc) => vec!["decree.publication_date:asc"],
        Some(Sort::Desc) => vec!["decree.publication_date:desc"],
        None => vec![],
    };

    let results: SearchResults<DecreeDelegate> = meilisearch_client
        .index(Index::Decrees.uid(parliament))
        .search()
        .with_filter(&meilisearch_filter)
        .with_sort(&sort)
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

    let updated_at =
        crate::meilisearch::get_update_time_of_index(redis_con, parliament, &Index::Decrees)
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
