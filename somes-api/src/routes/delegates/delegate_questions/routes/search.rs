use axum::{Json, extract::Query};
use combx::Index;
use meilisearch_sdk::search::SearchResults;
use serde::{Deserialize, Serialize};
use somes_meilisearch_filter::{FilterOptions, to_meilisearch_filters};

use crate::{
    DELEGATE_QUESTIONS_PER_PAGE, ParliamentCtx, Qs, RedisConnection,
    meilisearch::MeilisearchClient,
    routes::{
        FilterError,
        models::{PublicDelegateQuestion, PublicDelegateQuestionFilter},
    },
};

#[derive(Debug, Deserialize, Serialize)]
pub struct DelegateQuestionsWithMaxPage {
    pub delegate_questions: Vec<PublicDelegateQuestion>,
    pub entry_count: i64,
    pub max_page: i64,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

pub async fn delegate_questions_search(
    ParliamentCtx(parliament): ParliamentCtx,
    RedisConnection(mut redis_con): RedisConnection,
    MeilisearchClient(meilisearch_client): MeilisearchClient,
    Query(search_query): Query<somes_common_lib::SearchQuery>,
    Query(page): Query<somes_common_lib::Page>,
    Query(entry_count_per_page): Query<somes_common_lib::PageEntryCount>,
    Query(sort): Query<somes_common_lib::SortParams>,
    Query(date_range): Query<somes_common_lib::DateRangeQueryFilter>,
    Query(topics): Query<somes_common_lib::TopicsFilter>,
    Qs(delegate_question_filter): Qs<PublicDelegateQuestionFilter>,
) -> Result<Json<DelegateQuestionsWithMaxPage>, FilterError> {
    let mut filter_conditions = to_meilisearch_filters(
        &delegate_question_filter.filter_arguments(),
        &FilterOptions::default(),
    );

    if let Some(date_from) = date_range.date_from {
        filter_conditions.push(format!("created_at >= {:?}", date_from.to_string()));
    }
    if let Some(date_to) = date_range.date_to {
        filter_conditions.push(format!("created_at <= {:?}", date_to.to_string()));
    }

    if let Some(topics) = topics.filter_topics
        && !topics.is_empty()
    {
        let eurovoc_conditions = topics
            .iter()
            .map(|topic| format!("topics.id = {topic:?}"))
            .collect::<Vec<_>>()
            .join(" OR ");

        filter_conditions.push(format!("({eurovoc_conditions}) "));
    }

    let meilisearch_filter = filter_conditions.join(" AND ");

    log::info!("meilisearch filter: {meilisearch_filter}");

    let sort = match sort.sort {
        Some(somes_common_lib::Sort::Asc) => {
            vec!["created_at:asc"]
        }
        Some(somes_common_lib::Sort::Desc) => {
            vec!["created_at:desc"]
        }
        None => {
            vec![]
        }
    };

    let results: SearchResults<PublicDelegateQuestion> = meilisearch_client
        .index(Index::DelegateQuestions.uid(parliament))
        .search()
        .with_filter(&meilisearch_filter)
        .with_query(&search_query.search.unwrap_or_default())
        .with_hits_per_page(
            entry_count_per_page
                .entries_per_page
                .unwrap_or(DELEGATE_QUESTIONS_PER_PAGE.parse().unwrap_or(16)),
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

    let delegate_questions = results
        .hits
        .into_iter()
        .map(|hit| hit.result)
        .collect::<Vec<_>>();
    Ok(Json(DelegateQuestionsWithMaxPage {
        delegate_questions,
        entry_count: results.estimated_total_hits.unwrap_or(1) as i64,
        max_page,
        updated_at,
    }))
}
