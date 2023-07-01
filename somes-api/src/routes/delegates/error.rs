use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum DelegatesErrorResponse {
    DelegateResponseError,
    ProposalResponseError,
}

impl IntoResponse for DelegatesErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match self {
            DelegatesErrorResponse::DelegateResponseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "could not return delegates",
            ),
            DelegatesErrorResponse::ProposalResponseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "could not return proposals",
            ),
        };

        let body = Json(json!({
            "error": err_msg,
        }));

        (status_code, body).into_response()
    }
}
