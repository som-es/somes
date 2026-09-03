use combx::{Index, Parliament};
use common_scrapes::language::Language;
use meilisearch_sdk::client::Client;
use redis::aio::ConnectionManager;
use somes_common_lib::ToCompositeType;

use super::{index_settings, swap::rebuild_index_via_swap, update_time};
use crate::routes::{db::fetch_public_questions, models::PublicDelegateQuestion};

pub async fn update_delegate_questions_meilisearch_index(
    parliament: Parliament,
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    redis_con: &mut ConnectionManager,
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let filterable_fields = PublicDelegateQuestion::field_orders()
        .into_iter()
        .map(|field| field.to_string())
        .collect::<Vec<String>>();

    log::info!("Fetching all delegate questions..");

    let language = match parliament {
        Parliament::At => Language::De,
        Parliament::Eu => Language::En,
    };

    let all_delegate_questions = fetch_public_questions(pg_pool, None, language).await?;

    let index = Index::DelegateQuestions.uid(parliament);

    log::info!("Fetched all delegate questions");

    log::info!(
        "Uploading {} delegate_questions to meilisearch",
        all_delegate_questions.len()
    );

    let settings = index_settings(
        &filterable_fields
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>(),
        &["created_at"],
    );

    rebuild_index_via_swap(
        client,
        &index,
        &settings,
        &all_delegate_questions,
        Some("id"),
        Some(1000),
    )
    .await?;
    update_time::update_update_time_of_index(redis_con, parliament, &Index::DelegateQuestions)
        .await?;

    log::info!("Uploaded delegate_questions");
    Ok(())
}
