use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FilterError {
    #[error("Database failure: {0}")]
    SqlFailure(#[from] sqlx::Error),
    #[error("Redis failure: {0}")]
    RedisFailure(#[from] redis::RedisError),
    #[error("Meilisearch failure: {0}")]
    MeilisearchFailure(#[from] meilisearch_sdk::errors::Error),
    #[error("internal server error")]
    Internal,
    #[error("entries not found")]
    NotFound,
    #[error("Invalid page: {0}")]
    InvalidPage(u32),
    #[error("Invalid days, max days: {0}")]
    InvalidDays(u32),
}

impl IntoResponse for FilterError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match &self {
            FilterError::SqlFailure(_e) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            FilterError::RedisFailure(_e) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            FilterError::MeilisearchFailure(_e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            FilterError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            FilterError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            FilterError::InvalidPage(_page) => (StatusCode::BAD_REQUEST, self.to_string()),
            FilterError::InvalidDays(_page) => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        let body = Json(json!({
            "error": err_msg,
            "type": "FilterError",
            "field": format!("{:?}", self),
        }));

        (status_code, body).into_response()
    }
}
