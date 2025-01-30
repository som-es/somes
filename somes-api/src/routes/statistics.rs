use axum::Json;
use serde::{Deserialize, Serialize};
use somes_common_lib::LegisPeriod;
use utoipa::{IntoParams, ToSchema};
mod absences_per_age;
mod absences_per_delegate;
mod absences_per_gender;
mod absences_per_party;
mod absolute_majority_initiatives;
mod age_of_delegates;
mod age_per_party;
mod call_to_orders;
mod call_to_orders_by_delegate;
mod call_to_orders_per_age;
mod call_to_orders_per_gender;
mod call_to_orders_per_party;
mod complexity_at_age;
mod complexity_per_gender;
mod complexity_per_party;
mod complexity_per_person;
mod division_accuracy_score_per_age;
mod division_accuracy_score_per_delegate;
mod division_accuracy_score_per_gender;
mod division_accuracy_score_per_party;
mod error;
mod filtering;
mod speechtime_per_age;
mod speechtime_per_delegate;
mod speechtime_per_gender;
mod speechtime_per_party;
mod total_speeches_per_age;
mod total_speeches_per_delegate;
mod total_speeches_per_gender;
mod total_speeches_per_party;
mod votes_together;
mod absences_per_legis;
mod age_per_gender;
mod age_per_legis;
mod call_to_orders_per_legis;
mod complexity_per_legis;
mod division_accuracy_score_per_legis;
mod speechtime_per_legis;
mod total_speeches_per_legis;

pub use absences_per_age::*;
pub use absences_per_delegate::*;
pub use absences_per_gender::*;
pub use absences_per_party::*;
pub use absolute_majority_initiatives::*;
pub use age_of_delegates::*;
pub use age_per_party::*;
pub use call_to_orders::*;
pub use call_to_orders_by_delegate::*;
pub use call_to_orders_per_age::*;
pub use call_to_orders_per_gender::*;
pub use call_to_orders_per_party::*;
pub use complexity_at_age::*;
pub use complexity_per_gender::*;
pub use complexity_per_party::*;
pub use complexity_per_person::*;
pub use division_accuracy_score_per_age::*;
pub use division_accuracy_score_per_delegate::*;
pub use division_accuracy_score_per_gender::*;
pub use division_accuracy_score_per_party::*;
pub use speechtime_per_age::*;
pub use speechtime_per_delegate::*;
pub use speechtime_per_gender::*;
pub use speechtime_per_party::*;
pub use total_speeches_per_age::*;
pub use total_speeches_per_delegate::*;
pub use total_speeches_per_gender::*;
pub use total_speeches_per_party::*;
pub use votes_together::*;
pub use absences_per_delegate::*;
pub use absences_per_party::*;
pub use absences_per_age::*;
pub use absences_per_legis::*;
pub use age_per_gender::*;
pub use age_per_legis::*;
pub use call_to_orders_per_legis::*;
pub use complexity_per_legis::*;
pub use division_accuracy_score_per_legis::*;
pub use speechtime_per_legis::*;
use crate::{dataservice::{get_call_to_orders_per_party_delegates, get_delegates_by_call_to_orders, get_delegates_by_call_to_orders_by_legis_period, get_speakers_by_hours, get_speakers_by_hours_by_legis_period}, interact, model::{CallToOrdersPerPartyDelegates, DelegateByCallToOrders, SpeakerByHours}, DataserviceDbConnection};

pub use self::total_speeches_per_legis::*;

use self::error::StatisticsResponse;

#[utoipa::path(
    get,
    path = "/speakers_by_hours", 
    responses(
        (status = 200, description = "Returned speakers by hours successfully.", body = [Vec<SpeakerByHours>]), 
        (status = 400, description = "Invalid request", body = [StatisticsResponse]),
        (status = 500, description = "Internal server error", body = [StatisticsResponse])
    )
)]
pub async fn speakers_by_hours(
    DataserviceDbConnection(postgres_con): DataserviceDbConnection,
) -> Result<Json<Vec<SpeakerByHours>>, StatisticsResponse> {
    interact!(
        postgres_con,
        get_speakers_by_hours(postgres_con)
            .map_err(|_| StatisticsResponse::DbSelectFailure(None))
            .map(Json)
    )
    .map_err(|_| StatisticsResponse::DbSelectFailure(None))?
}

#[utoipa::path(
    post,
    path = "/speakers_by_hours_and_legis_period", 
    params(
        LegisPeriod
    ),
    responses(
        (status = 200, description = "Returned speakers by hours and legislative period successfully.", body = [Vec<SpeakerByHours>]), 
        (status = 400, description = "Invalid request", body = [StatisticsResponse]),
        (status = 500, description = "Internal server error", body = [StatisticsResponse])
    )
)]
pub async fn speakers_by_hours_and_legis_period(
    DataserviceDbConnection(postgres_con): DataserviceDbConnection,
    Json(legis_period): Json<LegisPeriod>,
) -> Result<Json<Vec<SpeakerByHours>>, StatisticsResponse> {
    postgres_con
        .interact(|con| {
            let legis_period = legis_period;
            get_speakers_by_hours_by_legis_period(con, &legis_period)
                .map_err(|_| StatisticsResponse::DbSelectFailure(None))
                .map(Json)
        })
        .await
        .map_err(|_| StatisticsResponse::DbSelectFailure(None))?
}

#[utoipa::path(
    get,
    path = "/delegates_by_call_to_orders", 
    responses(
        (status = 200, description = "Returned delegates by call to orders successfully.", body = [Vec<DelegateByCallToOrders>]), 
        (status = 400, description = "Invalid request", body = [StatisticsResponse]),
        (status = 500, description = "Internal server error", body = [StatisticsResponse])
    )
)]
pub async fn delegates_by_call_to_orders(
    DataserviceDbConnection(postgres_con): DataserviceDbConnection,
) -> Result<Json<Vec<DelegateByCallToOrders>>, StatisticsResponse> {
    postgres_con
        .interact(|con| {
            get_delegates_by_call_to_orders(con)
                .map_err(|_| StatisticsResponse::DbSelectFailure(None))
                .map(Json)
        })
        .await
        .map_err(|_| StatisticsResponse::DbSelectFailure(None))?
}
#[utoipa::path(
    post,
    path = "/delegates_by_call_to_orders_and_legis_period",
    params(
        LegisPeriod
    ),
    responses(
        (status = 200, description = "Returned delegates by call to orders and legis period successfully.", body = [Vec<DelegateByCallToOrders>]), 
        (status = 400, description = "Invalid request", body = [StatisticsResponse]),
        (status = 500, description = "Internal server error", body = [StatisticsResponse])
    )
)]
pub async fn delegates_by_call_to_orders_and_legis_period(
    DataserviceDbConnection(postgres_con): DataserviceDbConnection,
    Json(legis_period): Json<LegisPeriod>,
) -> Result<Json<Vec<DelegateByCallToOrders>>, StatisticsResponse> {
    postgres_con
        .interact(|con| {
            let legis_period = legis_period;
            get_delegates_by_call_to_orders_by_legis_period(con, &legis_period)
                .map_err(|_| StatisticsResponse::DbSelectFailure(None))
                .map(Json)
        })
        .await
        .map_err(|_| StatisticsResponse::DbSelectFailure(None))?
}

#[utoipa::path(
    get,
    path = "/call_to_orders_per_party_delegates", 
    responses(
        (status = 200, description = "Returned call to order per party delegates successfully.", body = [Vec<CallToOrdersPerPartyDelegates>]), 
        (status = 400, description = "Invalid request", body = [StatisticsResponse]),
        (status = 500, description = "Internal server error", body = [StatisticsResponse])
    )
)]
pub async fn call_to_orders_per_party_delegates(
    DataserviceDbConnection(postgres_con): DataserviceDbConnection,
) -> Result<Json<Vec<CallToOrdersPerPartyDelegates>>, StatisticsResponse> {
    postgres_con
        .interact(|con| {
            get_call_to_orders_per_party_delegates(con)
                .map_err(|_| StatisticsResponse::DbSelectFailure(None))
                .map(Json)
        })
        .await
        .map_err(|_| StatisticsResponse::DbSelectFailure(None))?
}
