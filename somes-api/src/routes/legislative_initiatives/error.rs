use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;
use utoipa::ToSchema;

use crate::GenericErrorResponse;

#[derive(ToSchema, Debug)]
pub enum LegisInitErrorResponse {
    LegisInit,
    LatestLegisInit,
    LatestVoteResults,
    VoteResultById,
    InvalidPage,
    GenericErrorResponse(GenericErrorResponse)
}

impl IntoResponse for LegisInitErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let (status_code, err_msg) = match self {
            LegisInitErrorResponse::LegisInit => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "could not return legislative initiatives",
                    ),
            LegisInitErrorResponse::LatestLegisInit => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "could not return latest legislative initiatives",
                    ),
            LegisInitErrorResponse::LatestVoteResults => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "could not return latest vote results",
                    ),
            LegisInitErrorResponse::InvalidPage => (StatusCode::BAD_REQUEST, "invalid page"),
            LegisInitErrorResponse::VoteResultById => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "could not return legislative init by id",
                    ),
            LegisInitErrorResponse::GenericErrorResponse(generic_error_response) => return generic_error_response.into_response(),
        };

        let body = Json(json!({
            "error": err_msg,
        }));

        (status_code, body).into_response()
    }
}
