use axum::routing::post;
use utoipa_axum::{router::OpenApiRouter, routes};

mod routes;
pub use routes::*;

use crate::AppState;
use somes_common_lib::*;

pub fn create_statistics_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        // Legislative Initiatives endpoints
        .routes(routes!(legislative_initiatives_without_simple_majority))
        .routes(routes!(legislative_initiative_outcomes_by_period))
        // Call to Orders endpoints
        .routes(routes!(call_to_orders_per_delegate))
        .route(
            CALL_TO_ORDERS_BY_DELEGATE,
            post(call_to_orders_per_delegate),
        )
        .routes(routes!(call_to_orders_per_party))
        .routes(routes!(call_to_orders_per_gender))
        .routes(routes!(call_to_orders_per_age))
        .routes(routes!(call_to_orders_per_legis))
        // Absences endpoints
        .routes(routes!(absences_per_delegate))
        .routes(routes!(absences_per_party))
        .routes(routes!(absences_per_gender))
        .routes(routes!(absences_per_age))
        .routes(routes!(absences_per_legis))
        // Activity endpoints
        .routes(routes!(activity_per_delegate))
        .routes(routes!(activity_per_party))
        .routes(routes!(activity_per_gender))
        .routes(routes!(activity_per_age))
        .routes(routes!(activity_per_legis))
        // Age endpoints
        .routes(routes!(age_of_delegates))
        .routes(routes!(age_per_party))
        .routes(routes!(age_per_gender))
        .routes(routes!(age_per_legis))
        .routes(routes!(age_per_age))
        // Complexity endpoints
        .routes(routes!(complexity_per_delegate))
        .routes(routes!(complexity_per_party))
        .routes(routes!(complexity_per_gender))
        .routes(routes!(complexity_at_age))
        .routes(routes!(complexity_per_legis))
        // Division Accuracy Score endpoints
        .routes(routes!(division_accuracy_score_per_delegate))
        // Keep the historical misspelling working for existing clients.
        .route(
            "/divison_accuracy_score_per_delegate",
            post(division_accuracy_score_per_delegate),
        )
        .routes(routes!(division_accuracy_score_per_party))
        .routes(routes!(division_accuracy_score_per_gender))
        .routes(routes!(division_accuracy_score_per_age))
        .routes(routes!(division_accuracy_score_per_legis))
        // Political Orientation - Is Left endpoints
        .routes(routes!(is_left_per_delegate))
        .routes(routes!(is_left_per_party))
        .routes(routes!(is_left_per_gender))
        .routes(routes!(is_left_per_age))
        .routes(routes!(is_left_per_legis))
        // Political Orientation - Is Right endpoints
        .routes(routes!(is_right_per_delegate))
        .routes(routes!(is_right_per_party))
        .routes(routes!(is_right_per_gender))
        .routes(routes!(is_right_per_age))
        .routes(routes!(is_right_per_legis))
        // Political Orientation - Is Liberal endpoints
        .routes(routes!(is_liberal_per_delegate))
        .routes(routes!(is_liberal_per_party))
        .routes(routes!(is_liberal_per_gender))
        .routes(routes!(is_liberal_per_age))
        .routes(routes!(is_liberal_per_legis))
        // Political Orientation - Is Authoritarian endpoints
        .routes(routes!(is_authoritarian_per_delegate))
        .routes(routes!(is_authoritarian_per_party))
        .routes(routes!(is_authoritarian_per_gender))
        .routes(routes!(is_authoritarian_per_age))
        .routes(routes!(is_authoritarian_per_legis))
        // Political Orientation - Combined Spectrum endpoints
        .routes(routes!(political_spectrum_per_delegate))
        .routes(routes!(political_spectrum_per_party))
        .routes(routes!(political_spectrum_per_gender))
        .routes(routes!(political_spectrum_per_age))
        // Speeches - Speechtime endpoints
        .routes(routes!(speechtime_per_delegate))
        .routes(routes!(speechtime_per_party))
        .routes(routes!(speechtime_per_gender))
        .routes(routes!(speechtime_per_age))
        .routes(routes!(speechtime_per_legis))
        .routes(routes!(latest_session_activity_overview))
        // Speeches - Total Speeches endpoints
        .routes(routes!(total_speeches_per_delegate))
        .routes(routes!(total_speeches_per_party))
        .routes(routes!(total_speeches_per_gender))
        .routes(routes!(total_speeches_per_age))
        .routes(routes!(total_speeches_per_legis))
        .routes(routes!(votes_together))
}
