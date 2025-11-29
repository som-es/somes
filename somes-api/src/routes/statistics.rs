use axum::{routing::post, Json, Router};
mod absences_per_age;
mod absences_per_delegate;
mod absences_per_gender;
mod absences_per_legis;
mod absences_per_party;
mod absolute_majority_initiatives;
mod activity_per_age;
mod activity_per_delegate;
mod activity_per_gender;
mod activity_per_legis;
mod activity_per_party;
mod age_of_delegates;
mod age_per_gender;
mod age_per_legis;
mod age_per_party;
mod call_to_orders;
mod call_to_orders_by_delegate;
mod call_to_orders_per_age;
mod call_to_orders_per_gender;
mod call_to_orders_per_legis;
mod call_to_orders_per_party;
mod complexity_at_age;
mod complexity_per_gender;
mod complexity_per_legis;
mod complexity_per_party;
mod complexity_per_person;
mod division_accuracy_score_per_age;
mod division_accuracy_score_per_delegate;
mod division_accuracy_score_per_gender;
mod division_accuracy_score_per_legis;
mod division_accuracy_score_per_party;
mod error;
pub mod filtering;
mod is_left_per_age;
mod is_left_per_delegate;
mod is_left_per_gender;
mod is_left_per_legis;
mod is_left_per_party;
mod is_liberal_per_age;
mod is_liberal_per_delegate;
mod is_liberal_per_gender;
mod is_liberal_per_legis;
mod is_liberal_per_party;
mod speechtime_per_age;
mod speechtime_per_delegate;
mod speechtime_per_gender;
mod speechtime_per_legis;
mod speechtime_per_party;
mod total_speeches_per_age;
mod total_speeches_per_delegate;
mod total_speeches_per_gender;
mod total_speeches_per_legis;
mod total_speeches_per_party;
mod votes_together;

pub use absences_per_age::*;
pub use absences_per_delegate::*;
pub use absences_per_gender::*;
pub use absences_per_legis::*;
pub use absences_per_party::*;
pub use absolute_majority_initiatives::*;
pub use activity_per_age::*;
pub use activity_per_delegate::*;
pub use activity_per_gender::*;
pub use activity_per_legis::*;
pub use activity_per_party::*;
pub use age_of_delegates::*;
pub use age_per_gender::*;
pub use age_per_legis::*;
pub use age_per_party::*;
pub use call_to_orders::*;
pub use call_to_orders_by_delegate::*;
pub use call_to_orders_per_age::*;
pub use call_to_orders_per_gender::*;
pub use call_to_orders_per_legis::*;
pub use call_to_orders_per_party::*;
pub use complexity_at_age::*;
pub use complexity_per_gender::*;
pub use complexity_per_legis::*;
pub use complexity_per_party::*;
pub use complexity_per_person::*;
pub use division_accuracy_score_per_age::*;
pub use division_accuracy_score_per_delegate::*;
pub use division_accuracy_score_per_gender::*;
pub use division_accuracy_score_per_legis::*;
pub use division_accuracy_score_per_party::*;
pub use is_left_per_age::*;
pub use is_left_per_delegate::*;
pub use is_left_per_gender::*;
pub use is_left_per_legis::*;
pub use is_left_per_party::*;
pub use is_liberal_per_age::*;
pub use is_liberal_per_delegate::*;
pub use is_liberal_per_gender::*;
pub use is_liberal_per_legis::*;
pub use is_liberal_per_party::*;
pub use speechtime_per_age::*;
pub use speechtime_per_delegate::*;
pub use speechtime_per_gender::*;
pub use speechtime_per_legis::*;
pub use speechtime_per_party::*;
pub use total_speeches_per_age::*;
pub use total_speeches_per_delegate::*;
pub use total_speeches_per_gender::*;
pub use total_speeches_per_party::*;
pub use votes_together::*;

use crate::server::AppState;

pub use self::total_speeches_per_legis::*;

use somes_common_lib::*;

pub fn create_statistics_router() -> Router<AppState> {
    Router::new()
        .route(
            DELEGATES_BY_CALL_TO_ORDERS,
            post(call_to_order_per_delegates),
        )
        .route(
            LEGISLATIVE_INITIATIVES_WITHOUT_SIMPLE_MAJORITY,
            post(legislative_initiatives_without_simple_majority),
        )
        .route(COMPLEXITY_PER_DELEGATE, post(complexity_per_delegate))
        .route(COMPLEXITY_PER_PARTY, post(complexity_per_party))
        .route(COMPLEXITY_PER_GENDER, post(complexity_per_gender))
        .route(COMPLEXITY_AT_AGE, post(complexity_at_age))
        .route(AGE_OF_DELEGATES, post(age_per_delegate))
        .route(AGE_PER_PARTY, post(age_per_party))
        .route(SPEECHTIME_PER_PARTY, post(speechtime_per_party))
        .route(SPEECHTIME_PER_DELEGATE, post(speechtime_per_delegate))
        .route(SPEECHTIME_PER_AGE, post(speechtime_per_age))
        .route(SPEECHTIME_PER_GENDER, post(speechtime_per_gender))
        .route(
            TOTAL_SPEECHES_PER_DELEGATE,
            post(total_speeches_per_delegate),
        )
        .route(TOTAL_SPEECHES_PER_PARTY, post(total_speeches_per_party))
        .route(TOTAL_SPEECHES_PER_GENDER, post(total_speeches_per_gender))
        .route(TOTAL_SPEECHES_PER_AGE, post(total_speeches_per_age))
        .route(
            CALL_TO_ORDERS_BY_DELEGATE,
            post(call_to_orders_per_delegate),
        )
        .route(CALL_TO_ORDERS_PER_PARTY, post(call_to_orders_per_party))
        .route(CALL_TO_ORDERS_PER_GENDER, post(call_to_orders_per_gender))
        .route(CALL_TO_ORDERS_PER_AGE, post(call_to_orders_per_age))
        .route(
            DIVISION_ACCURACY_SCORE_PER_DELEGATE,
            post(divison_accuracy_score_per_delegate),
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
        .route(VOTES_TOGETHER, post(votes_together))
        .route(ABSENCES_PER_DELEGATE, post(absences_per_delegate))
        .route(ABSENCES_PER_PARTY, post(absences_per_party))
        .route(ABSENCES_PER_GENDER, post(absences_per_gender))
        .route(ABSENCES_PER_AGE, post(absences_per_age))
        .route(ABSENCES_PER_LEGIS, post(absences_per_legis))
        .route(AGE_PER_GENDER, post(age_per_gender))
        .route(AGE_PER_LEGIS, post(age_per_legis))
        .route(CALL_TO_ORDERS_PER_LEGIS, post(call_to_orders_per_legis))
        .route(COMPLEXITY_PER_LEGIS, post(complexity_per_legis))
        .route(
            DIVISION_ACCURACY_SCORE_PER_LEGIS,
            post(division_accuracy_score_per_legis),
        )
        .route(SPEECHTIME_PER_LEGIS, post(speechtime_per_legis))
        .route(TOTAL_SPEECHES_PER_LEGIS, post(total_speeches_per_legis))
        .route(ACTIVITY_PER_DELEGATE, post(activity_per_delegate))
        .route(ACTIVITY_PER_PARTY, post(activity_per_party))
        .route(ACTIVITY_PER_GENDER, post(activity_per_gender))
        .route(ACTIVITY_PER_AGE, post(activity_per_age))
        .route(ACTIVITY_PER_LEGIS, post(activity_per_legis))
        .route(IS_LEFT_PER_DELEGATE, post(is_left_per_delegate))
        .route(IS_LEFT_PER_PARTY, post(is_left_per_party))
        .route(IS_LEFT_PER_GENDER, post(is_left_per_gender))
        .route(IS_LEFT_PER_AGE, post(is_left_per_age))
        .route(IS_LEFT_PER_LEGIS, post(is_left_per_legis))
        .route(IS_LIBERAL_PER_DELEGATE, post(is_liberal_per_delegate))
        .route(IS_LIBERAL_PER_PARTY, post(is_liberal_per_party))
        .route(IS_LIBERAL_PER_GENDER, post(is_liberal_per_gender))
        .route(IS_LIBERAL_PER_AGE, post(is_liberal_per_age))
        .route(IS_LIBERAL_PER_LEGIS, post(is_liberal_per_legis))
}
