use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;
use utoipa::ToSchema;

#[derive(ToSchema, Debug)]
pub enum DelegatesErrorResponse {
    DelegateResponseError,
    ProposalResponseError,
    DelegateInterestsResponseError,
    DelegateSpeechesError,
    DateOutOfRangeError,
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
            DelegatesErrorResponse::DelegateInterestsResponseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "could not return interests of delegates",
            ),
            DelegatesErrorResponse::DateOutOfRangeError => {
                (StatusCode::BAD_REQUEST, "date out of range")
            }
            DelegatesErrorResponse::DelegateSpeechesError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "cannot return delegate speeches")

            },
        };

        let body = Json(json!({
            "error": err_msg,
        }));

        (status_code, body).into_response()
    }
}
