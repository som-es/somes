use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum LegisInitErrorResponse {
    LegisInitResponseError,
}

impl IntoResponse for LegisInitErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match self {
            LegisInitErrorResponse::LegisInitResponseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "could not return legislative initiatives",
            ),
        };

        let body = Json(json!({
            "error": err_msg,
        }));

        (status_code, body).into_response()
    }
}
