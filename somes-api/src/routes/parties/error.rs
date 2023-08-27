use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;
use utoipa::ToSchema;

#[derive(ToSchema, Debug)]
pub enum PartiesErrorResponse {
    PartiesReturn,
}

impl IntoResponse for PartiesErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match self {
            PartiesErrorResponse::PartiesReturn => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "could not return parties",
            ),
        };

        let body = Json(json!({
            "error": err_msg,
        }));

        (status_code, body).into_response()
    }
}
