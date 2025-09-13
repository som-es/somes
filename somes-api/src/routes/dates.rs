use axum::{extract::Query, Json};
use chrono::{Months, NaiveDate};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use somes_common_lib::Date;

use crate::{today, GenericErrorResponse, PgPoolConnection};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlenarDate {
    pub date: NaiveDate,
}

pub async fn next_plenar_date(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<PlenarDate>, GenericErrorResponse> {
    let now = today();
    sqlx::query_as!(
        PlenarDate,
        "select date from dates where date >= $1 and appointment_type = 'Plenarsitzung' and committee = 'Nationalrat' order by date asc limit 1",
        now
    )
    .fetch_one(&pg)
    .await
    .map_err(|e| GenericErrorResponse::DbSelectFailure(Some(e)))
    .map(Json)
}

pub async fn plenar_dates(
    Query(date): Query<Date>,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<PlenarDate>>, GenericErrorResponse> {
    let date_before =
        date.at
            .checked_sub_months(Months::new(2))
            .ok_or(GenericErrorResponse::Custom((
                StatusCode::BAD_REQUEST,
                "Invalid date",
            )))?;
    let date_after =
        date.at
            .checked_add_months(Months::new(2))
            .ok_or(GenericErrorResponse::Custom((
                StatusCode::BAD_REQUEST,
                "Invalid date",
            )))?;

    sqlx::query_as!(PlenarDate,
        "select date from dates where date >= $1 and date <= $2 and appointment_type = 'Plenarsitzung' and committee = 'Nationalrat'",
        date_before,
        date_after
    )
    .fetch_all(&pg)
    .await
    .map_err(|e| GenericErrorResponse::DbSelectFailure(Some(e)))
    .map(Json)
}
