use std::ops::{Deref, DerefMut};

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde_json::json;
use somes_common_lib::errors::SignUpError;
use utoipa::ToSchema;

use crate::AuthError;

#[derive(Debug, ToSchema)]
pub enum LoginErrorResponse {
    PostgresConnection,
    RedisGetKeys,

}

impl IntoResponse for LoginErrorResponse {
    fn into_response(self) -> Response {
        match self {
            LoginErrorResponse::PostgresConnection => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "An internal server error occurred. Postgres connection failed!"})),
            ).into_response(),
            LoginErrorResponse::RedisGetKeys => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "An internal server error occurred. Redis active user matching failed!"})),
            ).into_response(),
        }
    }
}
