use std::fmt::Display;

use axum::{extract::Query, Json};
use dataservice::combx::{
    AiSummaryFilter, OptionalVoteResult, OptionalVoteResultFilter, TopicFilter,
};
use meilisearch_sdk::search::SearchResults;
use redis::aio::MultiplexedConnection;
use somes_common_lib::{AddonVoteResultFilter, Page};
use somes_meilisearch_filter::{to_meilisearch_filters, CombineOp, FilterOptions};

use crate::{
    meilisearch::MeilisearchClient,
    routes::{FilterError, VoteResultsWithMaxPage},
    Qs, RedisConnection, LEGIS_INITS_PER_PAGE,
};

pub async fn vote_results_by_search_route(
    MeilisearchClient(meilisearch_client): MeilisearchClient,
    RedisConnection(mut redis_con): RedisConnection,
    Query(search_query): Query<somes_common_lib::SearchQuery>,
    Query(page): Query<somes_common_lib::Page>,
    Query(legis_init_filter): Query<AddonVoteResultFilter>,
    Qs(optional_vote_result_filter): Qs<OptionalVoteResultFilter>,
) -> Result<Json<VoteResultsWithMaxPage>, FilterError> {
    meilisearch_for_vote_results(
        legis_init_filter.is_finished,
        meilisearch_client,
        search_query,
        page,
        legis_init_filter,
        optional_vote_result_filter,
        &mut redis_con,
    )
    .await
    .map(Json)
}

#[inline]
fn create_topic_filter<T: Display>(field: &str, filter_values: impl Iterator<Item = T>) -> String {
    filter_values
        .into_iter()
        .map(|filter_value| format!("{field} = '{filter_value}'"))
        .collect::<Vec<_>>()
        .join(" AND ")
}

async fn meilisearch_for_vote_results(
    is_finished: bool,
    meilisearch_client: meilisearch_sdk::client::Client,
    search_query: somes_common_lib::SearchQuery,
    page: Page,
    filter: AddonVoteResultFilter,
    vote_result_filter: OptionalVoteResultFilter,
    redis_con: &mut MultiplexedConnection,
) -> Result<VoteResultsWithMaxPage, FilterError> {
    // dbg!(&vote_result_filter);
    let mut filter_conditions = if is_finished {
        vec![r#"legislative_initiative.accepted IS NOT NULL"#.to_string()]
    } else {
        vec![r#"legislative_initiative.accepted IS NULL AND legislative_initiative.has_reference = false"#.to_string()]
    };

    // dataservice::combx::DbLegislativeInitiativeQueryFilter ;

    let legis_init_filters = to_meilisearch_filters(
        &vote_result_filter
            .legislative_initiative
            .as_ref()
            .map(|x| x.filter_arguments())
            .unwrap_or_default(),
        &FilterOptions {
            prefix: Some("legislative_initiative".to_string()),
            ..Default::default()
        },
    );
    filter_conditions.extend(legis_init_filters);
    let top_level_filters = to_meilisearch_filters(
        &vote_result_filter.filter_arguments(),
        &FilterOptions::default(),
    );
    filter_conditions.extend(top_level_filters);

    filter_conditions.extend(to_topics_filter(
        vote_result_filter
            .eurovoc_topics
            .unwrap_or_default()
            .as_slice(),
        "eurovoc_topics",
    ));

    filter_conditions.extend(to_topics_filter(
        vote_result_filter
            .other_keyword_topics
            .unwrap_or_default()
            .as_slice(),
        "other_keyword_topics",
    ));

    filter_conditions.extend(to_topics_filter(
        vote_result_filter.topics.unwrap_or_default().as_slice(),
        "topics",
    ));

    filter_conditions.extend(to_meilisearch_filters(
        &vote_result_filter
            .ai_summary
            .as_ref()
            .map(|ai_summary| ai_summary.filter_arguments())
            .unwrap_or_default(),
        &FilterOptions {
            prefix: Some("ai_summary".into()),
            ..Default::default()
        },
    ));

    filter_conditions.extend(to_meilisearch_filters(
        &vote_result_filter
            .issued_by_dels
            .unwrap_or_default()
            .iter()
            .map(|issued_by_del| issued_by_del.filter_arguments())
            .flatten()
            .collect::<Vec<_>>(),
        &FilterOptions {
            prefix: Some("issued_by_dels".into()),
            combine_op: CombineOp::Or,
        },
    ));

    if let Some(party_votes) = &filter.party_votes {
        filter_conditions.push(create_topic_filter(
            "meilisearch_helper.votes",
            party_votes
                .iter()
                .map(|vote| format!("{}{:?}", vote.party, vote.infavor)),
        ));
        // filter_conditions.push(create_party_vote_filter(
        //     "votes.party",
        //     "votes.infavor",
        //     party_votes.iter(),
        // ));
    }

    let meilisearch_filter = filter_conditions.join(" AND ");

    log::info!("vote results meilisearch filter: {meilisearch_filter}, {search_query:?}");

    let results: SearchResults<OptionalVoteResult> = meilisearch_client
        .index("vote_results")
        .search()
        .with_filter(&meilisearch_filter)
        .with_sort(&["legislative_initiative.nr_plenary_activity_date:desc"])
        .with_query(&search_query.search)
        .with_hits_per_page(LEGIS_INITS_PER_PAGE.parse().unwrap_or(16))
        .with_page(page.page as usize)
        .execute()
        .await?;

    let max_page = results.total_pages.unwrap_or(1) as i64;
    let vote_results = results
        .hits
        .into_iter()
        .map(|hit| hit.result)
        .collect::<Vec<_>>();

    // log::info!(
    //     "results: {:?}",
    //     vote_results
    //         .iter()
    //         .map(|x| x.legislative_initiative.created_at)
    //         .collect::<Vec<_>>()
    // );

    Ok(VoteResultsWithMaxPage {
        vote_results,
        entry_count: results.estimated_total_hits.unwrap_or(1) as i64,
        max_page,
        updated_at: crate::meilisearch::get_update_time_of_index(
            redis_con,
            &crate::meilisearch::Index::VoteResults,
        )
        .await
        .ok()
        .map(|date| date.naive_local()),
    })
}

fn to_topics_filter(topics: &[TopicFilter], prefix: &str) -> Vec<String> {
    let topics = topics
        .iter()
        .map(|topic| topic.filter_arguments())
        .flatten()
        .collect::<Vec<_>>();
    to_meilisearch_filters(
        &topics,
        &FilterOptions {
            combine_op: CombineOp::Or,
            prefix: Some(prefix.into()),
        },
    )
}
