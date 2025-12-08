use axum::{Error, Json, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::json;
use utoipa::ToSchema;

use crate::ErrorInfo;

#[derive(ToSchema, Debug)]
pub enum WaloErrorResponse {
    QuestionResponseError,
    ProposalResponseError,
    DelegateInterestsResponseError,
}

impl IntoResponse for WaloErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match &self {
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

        let body = Json(ErrorInfo {
            error: err_msg.to_string(),
            error_type: "WaloErrorResponse",
            field: format!("{:?}", self),
            meta: None,
        });

        (status_code, body).into_response()
    }
}
