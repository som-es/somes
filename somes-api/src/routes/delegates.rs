use std::collections::HashMap;

use crate::server::AppState;
use crate::PgPoolConnection;
use axum::routing::get;
use axum::Router;
use axum::{extract::Query, Json};
use somes_common_lib::{
    Delegate, DelegateById, FullMandate, InterestShare, ALL_ACTIVE, ALL_AT_DATE,
    ALL_AT_DATE_WITH_SEAT_INFO, DELEGATE_QA, EXTEND, ID, SEARCH, SPEECHES_PER_PAGE_ROUTE,
};

pub use error::*;
mod absences;
mod ai_chat;
mod delegate_political_position;
mod error;
mod interests;
mod left_right_topic_score;
mod named_votes;
mod routes;
mod speeches;
mod stance_topic_score;
pub use absences::*;
pub use ai_chat::*;
pub use delegate_political_position::*;
pub use interests::*;
pub use routes::*;
pub use speeches::*;
use sqlx::PgPool;

pub fn create_delegates_router() -> Router<AppState> {
    Router::new()
        .route(ALL_AT_DATE, get(delegates_at_route))
        .route(ID, get(delegate_by_id_path_route))
        .route(ALL_ACTIVE, get(active_delegates_route))
        .route(DELEGATE_QA, get(delegate_qa_route))
        .route(SEARCH, get(delegates_by_search_route))
        .route(
            SPEECHES_PER_PAGE_ROUTE,
            get(speeches_by_delegate_per_page_route),
        )
        .route(
            ALL_AT_DATE_WITH_SEAT_INFO,
            get(delegates_with_seats_near_date_route),
        )
        .route(EXTEND, get(extended_delegate_info_route))
        .nest("/gov_officials", create_gov_officials_router())
}

#[utoipa::path(
    get,
    params(
        DelegateById
    ),
    path = "/delegate_interests",
    responses(
        (status = 200, description = "Returned delegate interests successfully.", body = [Vec<InterestShare>]),
        // (status = 400, description = "Invalid request", body = [DelegatesErrorResponse]),
        // (status = 500, description = "Internal server error", body = [DelegatesErrorResponse])
    )
)]
#[inline]
pub async fn delegate_interests(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id): Query<DelegateById>,
) -> Result<Json<Vec<InterestShare>>, DelegateError> {
    Ok(
        extract_interests_of_delegate(delegate_by_id.delegate_id, &pg)
            .await
            .map(Json)?,
    )
}

pub async fn seats_route() -> Json<HashMap<String, Vec<u32>>> {
    Json(
        [
            ("XXVII".to_string(), vec![20, 27, 37, 43, 48, 54]),
            ("XXVIII".to_string(), vec![20, 28, 37, 43, 48, 54]),
        ]
        .into_iter()
        .collect(),
    )
}

pub async fn all_delegates(pg: &PgPool) -> sqlx::Result<Vec<Delegate>> {
    sqlx::query_as!(
        Delegate,
        "
            SELECT
                * from delegates_with_mandates
            WHERE
                -- return only delegates with at least a single 'is_nr' mandate
                EXISTS (
                SELECT 1
                FROM unnest(\"mandates: Vec<FullMandate>\") am
                WHERE am.is_nr
            );
        "
    )
    .fetch_all(pg)
    .await
}
