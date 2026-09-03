pub mod db;
mod mail;
pub mod models;
mod recipients;
mod routes;

use axum::{
    Router,
    routing::{get, post},
};
use routes::*;
use std::sync::Arc;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

use crate::AppState;

pub fn create_delegate_questions_router() -> Router<AppState> {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(15)
            .burst_size(1)
            .finish()
            .unwrap(),
    );
    Router::new()
        .route("/", get(all_delegate_questions_route))
        .route("/search", get(delegate_questions_search))
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

#[cfg(test)]
#[path = "delegate_questions/tests/delegate_questions.rs"]
mod tests;
