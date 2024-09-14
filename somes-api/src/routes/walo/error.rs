use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;
use utoipa::ToSchema;

#[derive(ToSchema, Debug)]
pub enum WaloErrorResponse {
    QuestionResponseError,
    ProposalResponseError,
    DelegateInterestsResponseError,
}

impl IntoResponse for WaloErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match self {
            WaloErrorResponse::QuestionResponseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "could not return questions",
            ),
            WaloErrorResponse::ProposalResponseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "could not return proposals",
            ),
            WaloErrorResponse::DelegateInterestsResponseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "could not return interests of delegates",
            ),
        };

        let body = Json(json!({
            "error": err_msg,
        }));

        (status_code, body).into_response()
    }
}
