use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum QuestionErrorResponse {
    DbInteraction,
    FetchDelegateContact,
    InvalidDelegate,
}

impl IntoResponse for QuestionErrorResponse {
    fn into_response(self) -> axum::response::Response {
        // rather returning an entire struct? like in signup?
        let (status_code, err_msg) = match self {
            QuestionErrorResponse::FetchDelegateContact => {
                (StatusCode::BAD_REQUEST, "Invalid verification id provided.")
            }
            QuestionErrorResponse::InvalidDelegate => {
                (StatusCode::INTERNAL_SERVER_ERROR, "verification failed")
            }
            QuestionErrorResponse::DbInteraction => {
                (StatusCode::INTERNAL_SERVER_ERROR, "database interaction failed")
            },
        };

        let body = Json(json!({
            "error": err_msg,
        }));

        (status_code, body).into_response()
    }
}
