use crate::{AppState, TopicsExtractor};
use crate::{PgPoolConnection, routes::DelegateError};
use axum::{Json, extract::Query};
use somes_common_lib::{DelegateById, PoliticalPosition};
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn create_political_analysis_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(political_position))
}

#[utoipa::path(
    get,
    path = "/political_position",
    tag = "delegates",
    params(DelegateById),
    responses(
        (status = 200, description = "Political position", body = Option<PoliticalPosition>),
    )
)]
pub async fn political_position(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id): Query<DelegateById>,
    TopicsExtractor(eurovoc_topics): TopicsExtractor,
) -> Result<Json<Option<PoliticalPosition>>, DelegateError> {
    use crate::routes::delegates::left_right_topic_score::extract_political_position_by_delegate;
    extract_political_position_by_delegate(
        &pg,
        delegate_by_id.delegate_id,
        &eurovoc_topics,
        common_scrapes::language::Language::De,
    )
    .await
    .map(Json)
    .map_err(|_| DelegateError::Internal)
}
