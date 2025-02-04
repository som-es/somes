use std::borrow::Cow;

use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum GenericErrorResponse {
    CustomString((StatusCode, String)),
    Custom((StatusCode, &'static str)),
    DbSelectFailure(Option<sqlx::Error>),
}

impl IntoResponse for GenericErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match self {
            GenericErrorResponse::DbSelectFailure(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Cow::Owned(format!("db error occured: {e:?}")),
            ),
            GenericErrorResponse::CustomString((status_code, reason)) => {
                (status_code, Cow::Owned(reason))
            }
            GenericErrorResponse::Custom((status_code, reason)) => {
                (status_code, Cow::Borrowed(reason))
            }
        };

        let body = Json(json!({
            "error": err_msg,
        }));

        (status_code, body).into_response()
    }
}
