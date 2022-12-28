use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum VerifyErrorResponse {
    InvalidVerificationID,
    VerificationError,
    UserCreationError,
}

impl IntoResponse for VerifyErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match self {
            VerifyErrorResponse::InvalidVerificationID => {
                (StatusCode::BAD_REQUEST, "Invalid verification id provided.")
            }
            VerifyErrorResponse::VerificationError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "verification failed")
            }
            VerifyErrorResponse::UserCreationError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "creating user failed")
            }
        };

        let body = Json(json!({
            "error": err_msg,
        }));

        (status_code, body).into_response()
    }
}
