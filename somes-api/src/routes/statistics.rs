use axum::{routing::post, Router};

mod routes;
pub use routes::*;

use crate::server::AppState;
use somes_common_lib::*;

pub fn create_statistics_router() -> Router<AppState> {
    Router::new()
        // Legislative Initiatives endpoints
        .route(
            LEGISLATIVE_INITIATIVES_WITHOUT_SIMPLE_MAJORITY,
            post(legislative_initiatives_without_simple_majority),
        )
        // Call to Orders endpoints
        .route(
            DELEGATES_BY_CALL_TO_ORDERS,
            post(call_to_orders_per_delegate),
        )
        .route(
            CALL_TO_ORDERS_BY_DELEGATE,
            post(call_to_orders_per_delegate),
        )
        .route(CALL_TO_ORDERS_PER_PARTY, post(call_to_orders_per_party))
        .route(CALL_TO_ORDERS_PER_GENDER, post(call_to_orders_per_gender))
        .route(CALL_TO_ORDERS_PER_AGE, post(call_to_orders_per_age))
        .route(CALL_TO_ORDERS_PER_LEGIS, post(call_to_orders_per_legis))
        // Absences endpoints
        .route(ABSENCES_PER_DELEGATE, post(absences_per_delegate))
        .route(ABSENCES_PER_PARTY, post(absences_per_party))
        .route(ABSENCES_PER_GENDER, post(absences_per_gender))
        .route(ABSENCES_PER_AGE, post(absences_per_age))
        .route(ABSENCES_PER_LEGIS, post(absences_per_legis))
        // Activity endpoints
        .route(ACTIVITY_PER_DELEGATE, post(activity_per_delegate))
        .route(ACTIVITY_PER_PARTY, post(activity_per_party))
        .route(ACTIVITY_PER_GENDER, post(activity_per_gender))
        .route(ACTIVITY_PER_AGE, post(activity_per_age))
        .route(ACTIVITY_PER_LEGIS, post(activity_per_legis))
        // Age endpoints
        .route(AGE_OF_DELEGATES, post(age_of_delegates))
        .route(AGE_PER_PARTY, post(age_per_party))
        .route(AGE_PER_GENDER, post(age_per_gender))
        .route(AGE_PER_LEGIS, post(age_per_legis))
        .route("/age_per_age", post(age_per_age))
        // Complexity endpoints
        .route(COMPLEXITY_PER_DELEGATE, post(complexity_per_delegate))
        .route(COMPLEXITY_PER_PARTY, post(complexity_per_party))
        .route(COMPLEXITY_PER_GENDER, post(complexity_per_gender))
        .route(COMPLEXITY_AT_AGE, post(complexity_at_age))
        .route(COMPLEXITY_PER_LEGIS, post(complexity_per_legis))
        // Division Accuracy Score endpoints
        .route(
            DIVISION_ACCURACY_SCORE_PER_DELEGATE,
            post(division_accuracy_score_per_delegate),
        )
        .route(
            DIVISION_ACCURACY_SCORE_PER_PARTY,
            post(division_accuracy_score_per_party),
        )
        .route(
            DIVISION_ACCURACY_SCORE_PER_GENDER,
            post(division_accuracy_score_per_gender),
        )
        .route(
            DIVISION_ACCURACY_SCORE_PER_AGE,
            post(division_accuracy_score_per_age),
        )
        .route(
            DIVISION_ACCURACY_SCORE_PER_LEGIS,
            post(division_accuracy_score_per_legis),
        )
        // Political Orientation - Is Left endpoints
        .route(IS_LEFT_PER_DELEGATE, post(is_left_per_delegate))
        .route(IS_LEFT_PER_PARTY, post(is_left_per_party))
        .route(IS_LEFT_PER_GENDER, post(is_left_per_gender))
        .route(IS_LEFT_PER_AGE, post(is_left_per_age))
        .route(IS_LEFT_PER_LEGIS, post(is_left_per_legis))
        // Political Orientation - Is Liberal endpoints
        .route(IS_LIBERAL_PER_DELEGATE, post(is_liberal_per_delegate))
        .route(IS_LIBERAL_PER_PARTY, post(is_liberal_per_party))
        .route(IS_LIBERAL_PER_GENDER, post(is_liberal_per_gender))
        .route(IS_LIBERAL_PER_AGE, post(is_liberal_per_age))
        .route(IS_LIBERAL_PER_LEGIS, post(is_liberal_per_legis))
        // Speeches - Speechtime endpoints
        .route(SPEECHTIME_PER_DELEGATE, post(speechtime_per_delegate))
        .route(SPEECHTIME_PER_PARTY, post(speechtime_per_party))
        .route(SPEECHTIME_PER_GENDER, post(speechtime_per_gender))
        .route(SPEECHTIME_PER_AGE, post(speechtime_per_age))
        .route(SPEECHTIME_PER_LEGIS, post(speechtime_per_legis))
        // Speeches - Total Speeches endpoints
        .route(
            TOTAL_SPEECHES_PER_DELEGATE,
            post(total_speeches_per_delegate),
        )
        .route(TOTAL_SPEECHES_PER_PARTY, post(total_speeches_per_party))
        .route(TOTAL_SPEECHES_PER_GENDER, post(total_speeches_per_gender))
        .route(TOTAL_SPEECHES_PER_AGE, post(total_speeches_per_age))
        .route(TOTAL_SPEECHES_PER_LEGIS, post(total_speeches_per_legis))
        .route(VOTES_TOGETHER, post(votes_together))
}
