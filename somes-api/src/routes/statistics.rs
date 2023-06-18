use axum::Json;
mod error;

use crate::{
    dataservice::{get_delegates_by_call_to_orders, get_speakers_by_hours},
    interact,
    model::{DelegateByCallToOrders, SpeakerByHours},
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
