use std::collections::HashMap;

use crate::AppState;
use crate::eu_hemicycle::EuHemicycle;
use crate::{ParliamentCtx, PgPoolConnection};
use axum::Router;
use axum::routing::get;
use axum::{Json, extract::Query};
use combx::{Delegate, FullMandate};
use somes_common_lib::{
    ALL_ACTIVE, ALL_AT_DATE, ALL_AT_DATE_WITH_SEAT_INFO, DelegateById, EXTEND, ID,
    INTERJECTIONS_ROUTE, InterestShare, PARLIAMENT_QA_ROUTE, SEARCH, SPEECHES_PER_PAGE_ROUTE,
};

pub use error::*;
mod absences;
mod ai_chat;
mod delegate_political_position;
mod error;
mod interests;
mod interjections;
mod issued_proposals;
mod left_right_topic_score;
mod parliamentary_qa;
mod routes;
mod stance_topic_score;
pub use absences::*;
pub use ai_chat::*;
pub use delegate_political_position::*;
pub use interests::*;
pub use interjections::*;
pub(crate) use issued_proposals::*;
pub use parliamentary_qa::*;
pub use routes::*;
use sqlx::PgPool;

pub fn create_delegates_router() -> Router<AppState> {
    Router::new()
        .route(ALL_AT_DATE, get(delegates_at_route))
        .route(ID, get(delegate_by_id_path_route))
        .route(ALL_ACTIVE, get(active_delegates_route))
        // .route(DELEGATE_QA, get(delegate_qa_route))
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
        .nest(INTERJECTIONS_ROUTE, create_delegate_interjections_router())
        .nest(PARLIAMENT_QA_ROUTE, create_delegate_pqa_router())
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

pub async fn seats_route(
    ParliamentCtx(parliament): ParliamentCtx,
    EuHemicycle(hemicycle): EuHemicycle,
) -> Json<HashMap<String, Vec<u32>>> {
    let eu_strasbourg_seats = hemicycle
        .circles
        .iter()
        .map(|circle| circle.slots_including_gaps)
        .collect();
    let seats = match parliament {
        combx::Parliament::At => [
            ("XXVII".to_string(), vec![20, 27, 37, 43, 48, 54]),
            ("XXVIII".to_string(), vec![20, 28, 37, 43, 48, 54]),
            ("NO_SEATS".to_string(), vec![18, 25, 29, 33, 37, 41]),
        ]
        .into_iter()
        .collect(),
        combx::Parliament::Eu => [
            (
                "NO_SEATS".to_string(),
                vec![20, 27, 37, 43, 48, 54, 59, 71, 83, 98, 115, 129],
            ),
            ("10".to_string(), eu_strasbourg_seats),
        ]
        .into_iter()
        .collect(),
    };
    Json(seats)
}

pub async fn all_delegates(pg: &PgPool) -> sqlx::Result<Vec<Delegate>> {
    sqlx::query_as!(
        Delegate,
        "
            SELECT
                * from delegates_with_mandates
            WHERE
                -- return only delegates with at least a single 'is_nr' or 'is_gov_official' mandate
                EXISTS (
                SELECT 1
                FROM unnest(\"mandates: Vec<FullMandate>\") am
                WHERE am.is_nr or am.is_gov_official
            );
        "
    )
    .fetch_all(pg)
    .await
}
