use std::borrow::Cow;

use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub error: String,
    pub error_type: &'static str,
    pub field: String,
    pub meta: Option<serde_json::Value>,
}

#[derive(Debug)]
pub enum GenericError {
    CustomString((StatusCode, String)),
    Custom((StatusCode, &'static str)),
    SqlFailure(Option<sqlx::Error>),
    RedisFailure(redis::RedisError),
    MeilisearchFailure(meilisearch_sdk::errors::Error),
}

impl IntoResponse for GenericError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match &self {
            GenericError::SqlFailure(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Cow::Owned(format!("db error occured: {e:?}")),
            ),
            GenericError::CustomString((status_code, reason)) => {
                (*status_code, Cow::Borrowed(reason.as_str()))
            }
            GenericError::Custom((status_code, reason)) => (*status_code, Cow::Borrowed(*reason)),
            GenericError::RedisFailure(redis_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Cow::Owned(format!("redis error occured: {redis_error:?}")),
            ),
            GenericError::MeilisearchFailure(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Cow::Owned(format!("meilisearch error occured: {error:?}")),
            ),
        };

        let body = Json(ErrorInfo {
            error: err_msg.to_string(),
            error_type: "GenericErrorResponse",
            field: format!("{:?}", self),
            meta: None,
        });

        (status_code, body).into_response()
    }
}
