use axum::{extract::Query, Json};
use combx::{meilisearch_filters_ai_summary, meilisearch_filters_gov_props, Index};
use meilisearch_sdk::search::SearchResults;
use somes_meilisearch_filter::{to_meilisearch_filters, FilterOptions};

use crate::{
    meilisearch::MeilisearchClient,
    routes::{
        FilterError, GovProposalDelegate, GovProposalDelegateFilter, GovProposalsWithMaxPage,
    },
    ParliamentCtx, Qs, RedisConnection, GOV_PROPS_PER_PAGE,
};

pub async fn gov_props_by_search_route(
    ParliamentCtx(parliament): ParliamentCtx,
    RedisConnection(mut redis_con): RedisConnection,
    MeilisearchClient(meilisearch_client): MeilisearchClient,
    Query(search_query): Query<somes_common_lib::SearchQuery>,
    Query(page): Query<somes_common_lib::Page>,
    Query(entry_count_per_page): Query<somes_common_lib::PageEntryCount>,
    Query(sort): Query<somes_common_lib::SortParams>,
    Query(date_range): Query<somes_common_lib::DateRangeQueryFilter>,
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
    if let Some(delegates) = &gov_prop_filter.delegates {
        let filter_args = delegates
            .iter()
            .map(|delegate| delegate.filter_arguments())
            .flatten()
            .collect::<Vec<_>>();

        filter_conditions.extend(to_meilisearch_filters(
            &filter_args,
            &FilterOptions {
                prefix: Some("delegates".into()),
                ..Default::default()
            },
        ));
    }

    if let Some(date_from) = date_range.date_from {
        filter_conditions.push(format!(
            "gov_proposal.ministrial_proposal.raw_data_created_at >= {:?}",
            date_from.to_string()
        ));
    }
    if let Some(date_to) = date_range.date_to {
        filter_conditions.push(format!(
            "gov_proposal.ministrial_proposal.raw_data_created_at <= {:?}",
            date_to.to_string()
        ));
    }

    let meilisearch_filter = filter_conditions.join(" AND ");

    log::info!("meilisearch filter: {meilisearch_filter}");

    let sort = match sort.sort {
        Some(somes_common_lib::Sort::Asc) => {
            vec!["gov_proposal.ministrial_proposal.raw_data_created_at:asc"]
        }
        Some(somes_common_lib::Sort::Desc) => {
            vec!["gov_proposal.ministrial_proposal.raw_data_created_at:desc"]
        }
        None => {
            vec![]
        }
    };

    let results: SearchResults<GovProposalDelegate> = meilisearch_client
        .index(Index::GovProposals.uid(parliament))
        .search()
        .with_filter(&meilisearch_filter)
        .with_query(&search_query.search.unwrap_or_default())
        .with_hits_per_page(
            entry_count_per_page
                .entries_per_page
                .unwrap_or(GOV_PROPS_PER_PAGE.parse().unwrap_or(12)),
        )
        .with_page(page.page as usize)
        .with_sort(&sort)
        .execute()
        .await?;

    let max_page = results.total_pages.unwrap_or(1) as i64;

    let updated_at = crate::meilisearch::get_update_time_of_index(
        &mut redis_con,
        parliament,
        &Index::GovProposals,
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
