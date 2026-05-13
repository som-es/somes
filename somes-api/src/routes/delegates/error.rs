use axum::{response::IntoResponse, Json};
use chrono::NaiveDate;
use reqwest::StatusCode;
use thiserror::Error;

use crate::ErrorInfo;

#[derive(Debug, Error)]
pub enum DelegateError {
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
    #[error("Invalid date: {0}")]
    DateOutOfRange(NaiveDate),
}

impl IntoResponse for DelegateError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg, field) = match &self {
            DelegateError::SqlFailure(e) => {
                log::error!("delegate db error occurred: {e:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    "SqlFailure",
                )
            }
            DelegateError::RedisFailure(e) => {
                log::error!("delegate redis error occurred: {e:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    "RedisFailure",
                )
            }
            DelegateError::MeilisearchFailure(e) => {
                log::error!("delegate meilisearch error occurred: {e:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    "MeilisearchFailure",
                )
            }
            DelegateError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                self.to_string(),
                "Internal",
            ),
            DelegateError::NotFound => (StatusCode::NOT_FOUND, self.to_string(), "NotFound"),
            DelegateError::InvalidPage(_page) => {
                (StatusCode::BAD_REQUEST, self.to_string(), "InvalidPage")
            }
            DelegateError::DateOutOfRange(_date) => {
                (StatusCode::BAD_REQUEST, self.to_string(), "DateOutOfRange")
            }
        };

        let body = Json(ErrorInfo {
            error: err_msg,
            error_type: "DelegateError",
            field: field.to_string(),
            meta: None,
        });

        (status_code, body).into_response()
    }
}
