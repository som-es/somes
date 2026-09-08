use axum::{Json, response::IntoResponse};
use reqwest::StatusCode;

use crate::ErrorInfo;

#[derive(Debug)]
pub enum StatisticsResponse {
    DbSelectFailure(Option<sqlx::Error>),
}

impl IntoResponse for StatisticsResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg, field) = match &self {
            StatisticsResponse::DbSelectFailure(e) => {
                log::error!("statistics db error occurred: {e:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error",
                    "DbSelectFailure",
                )
            }
        };

        let body = Json(ErrorInfo {
            error: err_msg.to_string(),
            error_type: "StatisticsResponse",
            field: field.to_string(),
            meta: None,
        });

        (status_code, body).into_response()
    }
}
