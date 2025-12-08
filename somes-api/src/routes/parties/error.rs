use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use thiserror::Error;

use crate::ErrorInfo;

#[derive(Error, Debug)]
pub enum PartiesErrorResponse {
    #[error("Database failure: {0}")]
    SqlFailure(#[from] sqlx::Error),
}

impl IntoResponse for PartiesErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match &self {
            PartiesErrorResponse::SqlFailure(ref e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };

        let body = Json(ErrorInfo {
            error: err_msg.to_string(),
            error_type: "PartiesErrorResponse",
            field: format!("{:?}", self),
            meta: None,
        });

        (status_code, body).into_response()
    }
}
