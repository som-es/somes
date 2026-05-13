use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;
use thiserror::Error;

use crate::ErrorInfo;

#[derive(Debug, Error)]
pub enum FilterError {
    #[error("Database failure")]
    SqlFailure(#[from] sqlx::Error),
    #[error("Redis failure")]
    RedisFailure(#[from] redis::RedisError),
    #[error("Meilisearch failure")]
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
        let (status_code, err_msg, field) = match &self {
            FilterError::SqlFailure(e) => {
                log::error!("vote result db error occurred: {e:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    "SqlFailure",
                )
            }
            FilterError::RedisFailure(e) => {
                log::error!("vote result redis error occurred: {e:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    "RedisFailure",
                )
            }
            FilterError::MeilisearchFailure(e) => {
                log::error!("vote result meilisearch error occurred: {e:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    "MeilisearchFailure",
                )
            }
            FilterError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                self.to_string(),
                "Internal",
            ),
            FilterError::NotFound => (StatusCode::NOT_FOUND, self.to_string(), "NotFound"),
            FilterError::InvalidPage(_page) => {
                (StatusCode::BAD_REQUEST, self.to_string(), "InvalidPage")
            }
            FilterError::InvalidDays(_page) => {
                (StatusCode::BAD_REQUEST, self.to_string(), "InvalidDays")
            }
        };

        let body = Json(ErrorInfo {
            error: err_msg,
            error_type: "FilterError",
            field: field.to_string(),
            meta: None,
        });
        (status_code, body).into_response()
    }
}
