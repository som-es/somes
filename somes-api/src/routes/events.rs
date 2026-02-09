use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::server::AppState;

pub async fn create_event_route() {}
pub async fn delete_event_route() {}
pub async fn update_event_route() {}
pub async fn all_events_route() {}

pub fn create_delegates_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_event_route))
        .route("/delete", delete(delete_event_route))
        .route("/update", put(update_event_route))
        .route("/", get(all_events_route))
}
