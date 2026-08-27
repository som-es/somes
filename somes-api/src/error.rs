use axum::{Json, response::IntoResponse};
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
        let (status_code, err_msg, field) = match &self {
            GenericError::SqlFailure(e) => {
                log::error!("db error occurred: {e:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error",
                    "SqlFailure",
                )
            }
            GenericError::CustomString((status_code, reason)) => {
                (*status_code, reason.as_str(), "CustomString")
            }
            GenericError::Custom((status_code, reason)) => (*status_code, *reason, "Custom"),
            GenericError::RedisFailure(redis_error) => {
                log::error!("redis error occurred: {redis_error:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error",
                    "RedisFailure",
                )
            }
            GenericError::MeilisearchFailure(error) => {
                log::error!("meilisearch error occurred: {error:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error",
                    "MeilisearchFailure",
                )
            }
        };

        let body = Json(ErrorInfo {
            error: err_msg.to_string(),
            error_type: "GenericErrorResponse",
            field: field.to_string(),
            meta: None,
        });

        (status_code, body).into_response()
    }
}
