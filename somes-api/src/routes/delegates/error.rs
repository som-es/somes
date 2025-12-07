use std::borrow::Cow;

use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum DelegatesErrorResponse {
    DelegateResponseError,
    ProposalResponseError,
    DelegateInterestsResponseError,
    DelegateSpeechesError,
    DateOutOfRangeError,
    DbSelectFailure(Option<sqlx::Error>),
    Custom(String),
}

impl IntoResponse for DelegatesErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match &self {
            DelegatesErrorResponse::DelegateResponseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Cow::Borrowed("could not return delegates"),
            ),
            DelegatesErrorResponse::ProposalResponseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Cow::Borrowed("could not return proposals"),
            ),
            DelegatesErrorResponse::DelegateInterestsResponseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Cow::Borrowed("could not return interests of delegates"),
            ),
            DelegatesErrorResponse::DateOutOfRangeError => {
                (StatusCode::BAD_REQUEST, Cow::Borrowed("date out of range"))
            }
            DelegatesErrorResponse::DelegateSpeechesError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Cow::Borrowed("cannot return delegate speeches"),
            ),
            DelegatesErrorResponse::DbSelectFailure(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Cow::Owned(format!("db error occured: {e:?}")),
            ),
            DelegatesErrorResponse::Custom(e) => {
                (StatusCode::BAD_REQUEST, Cow::Borrowed(e.as_str()))
            }
        };

        let body = Json(json!({
            "error": err_msg,
        }));

        (status_code, body).into_response()
    }
}
