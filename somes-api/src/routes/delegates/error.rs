use std::borrow::Cow;

use axum::{response::IntoResponse, Json};
use chrono::NaiveDate;
use reqwest::StatusCode;
use serde_json::json;
use thiserror::Error;

use crate::ErrorInfo;

#[derive(Debug, Error)]
pub enum DelegateError {
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
    #[error("Invalid date: {0}")]
    DateOutOfRange(NaiveDate),
}

impl IntoResponse for DelegateError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match &self {
            DelegateError::SqlFailure(_e) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            DelegateError::RedisFailure(_e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            DelegateError::MeilisearchFailure(_e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            DelegateError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            DelegateError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            DelegateError::InvalidPage(_page) => (StatusCode::BAD_REQUEST, self.to_string()),
            DelegateError::DateOutOfRange(_date) => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        let body = Json(ErrorInfo {
            error: err_msg,
            error_type: "DelegateError",
            field: format!("{:?}", self),
            meta: None,
        });

        (status_code, body).into_response()
    }
}
