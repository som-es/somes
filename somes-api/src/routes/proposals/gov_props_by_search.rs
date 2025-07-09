use axum::{extract::Query, Json};
use meilisearch_sdk::search::SearchResults;
use reqwest::StatusCode;
use somes_common_lib::GovPropFilter;

use crate::{
    meilisearch::MeilisearchClient, routes::GovProposalDelegate, GenericErrorResponse,
    RedisConnection, GOV_PROPS_PER_PAGE,
};

use super::GovProposalsWithMaxPage;

pub async fn gov_props_by_search(
    RedisConnection(mut redis_con): RedisConnection,
    MeilisearchClient(meilisearch_client): MeilisearchClient,
    Query(search_query): Query<somes_common_lib::SearchQuery>,
    Query(page): Query<somes_common_lib::Page>,
    Json(legis_init_filter): Json<Option<GovPropFilter>>,
) -> Result<Json<GovProposalsWithMaxPage>, GenericErrorResponse> {
    let mut meilisearch_filter = String::new();
    if let Some(filter) = legis_init_filter {
        let mut filter_conditions = Vec::new();

        if let Some(ref legis_period) = filter.legis_period {
            filter_conditions.push(format!(
                "gov_proposal.ministrial_proposal.gp = '{}'",
                legis_period
            ));
        }
        if let Some(is_named_vote) = filter.has_vote_result {
            filter_conditions.push(format!(
                "gov_proposal.ministrial_proposal.has_vote_result = {}",
                is_named_vote
            ));
        }
        meilisearch_filter = filter_conditions.join(" AND ")
    }

    let results: SearchResults<GovProposalDelegate> = meilisearch_client
        .index("gov_props")
        .search()
        .with_filter(&meilisearch_filter)
        .with_query(&search_query.search)
        .with_hits_per_page(GOV_PROPS_PER_PAGE.parse().unwrap_or(12))
        .with_page(page.page as usize)
        .with_sort(&["gov_proposal.ministrial_proposal.created_at:desc"])
        .execute()
        .await
        .map_err(|e| {
            GenericErrorResponse::CustomString((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Cannot find gov proposals: {e:?}"),
            ))
        })?;

    let max_page = results.total_pages.unwrap_or(1) as i64;

    let gov_proposals = results
        .hits
        .into_iter()
        .map(|hit| hit.result)
        .collect::<Vec<_>>();
    Ok(Json(GovProposalsWithMaxPage {
        gov_proposals,
        entry_count: results.estimated_total_hits.unwrap_or(1) as i64,
        max_page,
    }))
}
