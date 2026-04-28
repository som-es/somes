use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;

use crate::ErrorInfo;

#[derive(Debug)]
pub enum StatisticsResponse {
    DbSelectFailure(Option<sqlx::Error>),
}

impl IntoResponse for StatisticsResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match &self {
            StatisticsResponse::DbSelectFailure(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("db error occured: {e:?}"),
            ),
        };

        let body = Json(ErrorInfo {
            error: err_msg,
            error_type: "StatisticsResponse",
            field: format!("{:?}", self),
            meta: None,
        });

        (status_code, body).into_response()
    }
}
