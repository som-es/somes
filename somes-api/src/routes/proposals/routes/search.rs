use axum::{extract::Query, Json};
use dataservice::combx::{
    meilisearch_filters_gov_props, meilisearch_filters_vote_result, OptionalGovProposalFilter,
};
use meilisearch_sdk::search::SearchResults;
use somes_meilisearch_filter::{to_meilisearch_filters, FilterOptions};

use crate::{
    meilisearch::MeilisearchClient,
    routes::{
        FilterError, GovProposalDelegate, GovProposalDelegateFilter, GovProposalsWithMaxPage,
    },
    Qs, RedisConnection, GOV_PROPS_PER_PAGE,
};

pub async fn gov_props_by_search_route(
    RedisConnection(mut redis_con): RedisConnection,
    MeilisearchClient(meilisearch_client): MeilisearchClient,
    Query(search_query): Query<somes_common_lib::SearchQuery>,
    Query(page): Query<somes_common_lib::Page>,
    Qs(gov_prop_filter): Qs<GovProposalDelegateFilter>,
) -> Result<Json<GovProposalsWithMaxPage>, FilterError> {
    let mut filter_conditions = to_meilisearch_filters(
        &gov_prop_filter.filter_arguments(),
        &FilterOptions::default(),
    );

    if let Some(gov_proposal_filter) = gov_prop_filter.gov_proposal {
        filter_conditions.extend(meilisearch_filters_gov_props(
            gov_proposal_filter,
            Some("gov_proposal"),
        ));
    }
    if let Some(delegate) = &gov_prop_filter.delegate {
        filter_conditions.extend(to_meilisearch_filters(
            &delegate.filter_arguments(),
            &FilterOptions {
                prefix: Some("delegate".into()),
                ..Default::default()
            },
        ));
    }

    let meilisearch_filter = filter_conditions.join(" AND ");

    log::info!("meilisearch filter: {meilisearch_filter}");

    let results: SearchResults<GovProposalDelegate> = meilisearch_client
        .index("gov_props")
        .search()
        .with_filter(&meilisearch_filter)
        .with_query(&search_query.search.unwrap_or_default())
        .with_hits_per_page(GOV_PROPS_PER_PAGE.parse().unwrap_or(12))
        .with_page(page.page as usize)
        .with_sort(&["gov_proposal.ministrial_proposal.raw_data_created_at:desc"])
        .execute()
        .await?;

    let max_page = results.total_pages.unwrap_or(1) as i64;

    let updated_at = crate::meilisearch::get_update_time_of_index(
        &mut redis_con,
        &crate::meilisearch::Index::GovProposals,
    )
    .await
    .ok()
    .map(|date| date.naive_local());

    let gov_proposals = results
        .hits
        .into_iter()
        .map(|hit| hit.result)
        .collect::<Vec<_>>();
    Ok(Json(GovProposalsWithMaxPage {
        gov_proposals,
        entry_count: results.estimated_total_hits.unwrap_or(1) as i64,
        max_page,
        updated_at,
    }))
}
