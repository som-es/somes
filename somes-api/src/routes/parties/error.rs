use axum::{Json, response::IntoResponse};
use reqwest::StatusCode;
use thiserror::Error;

use crate::ErrorInfo;

#[derive(Error, Debug)]
pub enum PartiesErrorResponse {
    #[error("Database failure")]
    SqlFailure(#[from] sqlx::Error),
}

impl IntoResponse for PartiesErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg, field) = match &self {
            PartiesErrorResponse::SqlFailure(e) => {
                log::error!("parties db error occurred: {e:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error",
                    "SqlFailure",
                )
            }
        };

        let body = Json(ErrorInfo {
            error: err_msg.to_string(),
            error_type: "PartiesErrorResponse",
            field: field.to_string(),
            meta: None,
        });

        (status_code, body).into_response()
    }
}
