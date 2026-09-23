pub mod db;
mod mail;
pub mod models;
mod recipients;
mod routes;

use axum::{
    Router,
    routing::{get, post},
};
use combx::{Index, Parliament};
use common_scrapes::language::Language;
use routes::*;
use std::sync::Arc;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use utoipa_axum::router::OpenApiRouter;

use crate::{AppState, GenericError, routes::db::find_public_question};

pub fn create_delegate_questions_router() -> OpenApiRouter<AppState> {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(15)
            .burst_size(1)
            .finish()
            .unwrap(),
    );
    OpenApiRouter::new()
        .route("/", get(all_delegate_questions_route))
        .route("/search", get(delegate_questions_search))
        .route("/status", get(status_info_route))
        .route("/status/toggle", post(toggle_status_route))
        .route(
            "/delegate/{delegate_id}",
            get(delegate_questions_route)
                .post(post(ask_delegate_question_route).layer(GovernorLayer::new(governor_conf))),
        )
        .route(
            "/delegate/{delegate_id}/question_recipient",
            get(delegate_question_recipient_route),
        )
        .route("/pending", get(pending_delegate_questions_route))
        .route(
            "/{question_id}/approve",
            post(approve_delegate_question_route),
        )
        .route(
            "/{question_id}/reject",
            post(reject_delegate_question_route),
        )
        .route(
            "/{question_id}",
            get(delegate_question_by_id_route).patch(update_delegate_question_route),
        )
}

pub async fn update_question_in_meilisearch(
    meilisearch_client: &meilisearch_sdk::client::Client,
    pg: &sqlx::Pool<sqlx::Postgres>,
    parliament: Parliament,
    question_id: i64,
) -> Result<(), GenericError> {
    let language = match parliament {
        Parliament::At => Language::De,
        Parliament::Eu => Language::En,
    };
    let question = find_public_question(pg, question_id, language).await?;
    meilisearch_client
        .index(Index::DelegateQuestions.as_str())
        .add_documents(&[question], None)
        .await
        .map_err(|e| GenericError::MeilisearchFailure(e))?;
    Ok(())
}
#[cfg(test)]
#[path = "delegate_questions/tests/delegate_questions.rs"]
mod tests;
