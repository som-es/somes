use axum::Json;
use serde::{Deserialize, Serialize};
mod error;

use crate::{
    dataservice::{
        get_call_to_orders_per_party_delegates, get_delegates_by_call_to_orders,
        get_speakers_by_hours, get_delegates_by_call_to_orders_by_legis_period,
    },
    interact,
    model::{CallToOrdersPerPartyDelegates, DelegateByCallToOrders, SpeakerByHours},
    DataserviceDbConnection,
};

use self::error::StatisticsResponse;

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

pub async fn delegate_by_call_to_orders(
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

pub async fn delegates_by_call_to_orders_by_legis_period(
    Json(legis_period): Json<LegisPeriod>,
    DataserviceDbConnection(postgres_con): DataserviceDbConnection,
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct LegisPeriod {
    pub period: String,
}

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
