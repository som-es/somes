use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum StatisticsResponse {
    DbSelectFailure,
}

impl IntoResponse for StatisticsResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match self {
            StatisticsResponse::DbSelectFailure => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "could not return speakery by hours",
            ),
        };

        let body = Json(json!({
            "error": err_msg,
        }));

        (status_code, body).into_response()
    }
}
