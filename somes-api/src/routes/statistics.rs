use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
mod error;

use crate::{
    dataservice::{
        get_call_to_orders_per_party_delegates, get_delegates_by_call_to_orders,
        get_delegates_by_call_to_orders_by_legis_period, get_speakers_by_hours,
        get_speakers_by_hours_by_legis_period,
    },
    interact,
    model::{CallToOrdersPerPartyDelegates, DelegateByCallToOrders, SpeakerByHours},
    DataserviceDbConnection,
};

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
            .map_err(|_| StatisticsResponse::DbSelectFailure)
            .map(Json)
    )
    .map_err(|_| StatisticsResponse::DbSelectFailure)?
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
                .map_err(|_| StatisticsResponse::DbSelectFailure)
                .map(Json)
        })
        .await
        .map_err(|_| StatisticsResponse::DbSelectFailure)?
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
                .map_err(|_| StatisticsResponse::DbSelectFailure)
                .map(Json)
        })
        .await
        .map_err(|_| StatisticsResponse::DbSelectFailure)?
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
                .map_err(|_| StatisticsResponse::DbSelectFailure)
                .map(Json)
        })
        .await
        .map_err(|_| StatisticsResponse::DbSelectFailure)?
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
                .map_err(|_| StatisticsResponse::DbSelectFailure)
                .map(Json)
        })
        .await
        .map_err(|_| StatisticsResponse::DbSelectFailure)?
}
