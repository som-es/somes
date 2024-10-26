use std::collections::HashMap;

use axum::{extract::Query, Json};
use chrono::NaiveDate;
use dataservice::db::models::{DbDelegate, DbProposalQuery};
use serde::{Deserialize, Serialize};
use somes_common_lib::{Date, DelegateById, InterestShare};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::{
    dataservice::{get_delegate, get_delegates, get_proposals},
    DataserviceDbConnection, PgPoolConnection,
};

pub use error::*;
mod error;
mod interests;
pub use interests::*;

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct Delegate {
    pub id: i32,
    pub name: String,
    pub party: Option<String>,
    pub current_party: Option<String>,
    pub image_url: Option<String>,
    pub constituency: Option<String>,
    pub council: Option<String>,
    pub seat_row: Option<i32>,
    pub seat_col: Option<i32>,
    pub gender: Option<String>,
    pub is_active: Option<bool>,
    pub birthdate: Option<NaiveDate>,
    pub active_since: NaiveDate,
    pub divisions: Option<Vec<String>>,
}

#[utoipa::path(
    get,
    params(
        DelegateById
    ),
    path = "/delegate_interests", 
    responses(
        (status = 200, description = "Returned delegate interests successfully.", body = [Vec<InterestShare>]), 
        (status = 400, description = "Invalid request", body = [DelegatesErrorResponse]),
        (status = 500, description = "Internal server error", body = [DelegatesErrorResponse])
    )
)]
#[inline]
pub async fn delegate_interests(
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id): Query<DelegateById>,
) -> Result<Json<Vec<InterestShare>>, DelegatesErrorResponse> {
    extract_interests_of_delegate(delegate_by_id.delegate_id, &pg)
        .await
        .map(Json)
        .map_err(|_| DelegatesErrorResponse::DelegateInterestsResponseError)
}

#[utoipa::path(
    get,
    params(
        DelegateById
    ),
    path = "/delegate", 
    responses(
        (status = 200, description = "Returned delegate successfully.", body = [DbDelegate]), 
        (status = 400, description = "Invalid request", body = [DelegatesErrorResponse]),
        (status = 500, description = "Internal server error", body = [DelegatesErrorResponse])
    )
)]
pub async fn delegate(
    // DataserviceDbConnection(con): DataserviceDbConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id): Query<DelegateById>,
) -> Result<Json<Delegate>, DelegatesErrorResponse> {
    sqlx::query_as!(
        Delegate,
        "
        SELECT 
            delegates.id, 
            delegates.name, 
            delegates.party, 
            delegates.party as current_party, 
            delegates.image_url, 
            delegates.constituency, 
            delegates.council, 
            delegates.seat_row, 
            delegates.seat_col, 
            delegates.gender, 
            delegates.is_active, 
            delegates.birthdate, 
            mandates.start_date AS active_since,
            COALESCE(divisions.division_array, '{}') AS divisions
        FROM 
            mandates 
        INNER JOIN 
            delegates ON delegates.id = mandates.delegate_id 
        LEFT JOIN 
            (SELECT 
                delegate_id, 
                ARRAY_AGG(division) AS division_array 
            FROM 
                delegates_divisions 
            GROUP BY 
                delegate_id) AS divisions 
            ON delegates.id = divisions.delegate_id
        WHERE 
            (mandates.name LIKE '%Abgeordnete%' OR mandates.name LIKE '%minister%') 
            AND mandates.end_date IS NULL 
            AND delegates.id = $1;
",
        delegate_by_id.delegate_id
    )
    .fetch_one(&pg)
    .await
    .map(Json)
    .map_err(|_| DelegatesErrorResponse::DelegateResponseError)
    // con.interact(move |con| {
    //     get_delegate(con, delegate_by_id.delegate_id)
    //         .map(Json)
    //         .map_err(|_| DelegatesErrorResponse::DelegateResponseError)
    // })
    // .await
    // .map_err(|_| DelegatesErrorResponse::DelegateResponseError)?
}

#[utoipa::path(
    get,
    path = "/delegates", 
    responses(
        (status = 200, description = "Returned delegates successfully.", body = [Vec<DbDelegate>]), 
        (status = 400, description = "Invalid request", body = [DelegatesErrorResponse]),
        (status = 500, description = "Internal server error", body = [DelegatesErrorResponse])
    )
)]
pub async fn delegates(
    // DataserviceDbConnection(con): DataserviceDbConnection,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<Delegate>>, DelegatesErrorResponse> {
    sqlx::query_as!(
        Delegate,
        "
        SELECT 
            delegates.id, 
            delegates.name, 
            delegates.party, 
            delegates.party as current_party, 
            delegates.image_url, 
            delegates.constituency, 
            delegates.council, 
            delegates.seat_row, 
            delegates.seat_col, 
            delegates.gender, 
            delegates.is_active, 
            delegates.birthdate, 
            mandates.start_date AS active_since,
            COALESCE(divisions.division_array, '{}') AS divisions
        FROM 
            mandates 
        INNER JOIN 
            delegates ON delegates.id = mandates.delegate_id 
        LEFT JOIN 
            (SELECT 
                delegate_id, 
                ARRAY_AGG(division) AS division_array 
            FROM 
                delegates_divisions 
            GROUP BY 
                delegate_id) AS divisions 
            ON delegates.id = divisions.delegate_id
        WHERE 
            (mandates.name LIKE '%Abgeordnete%' OR mandates.name LIKE '%minister%') 
            AND mandates.end_date IS NULL;
        "
    )
    .fetch_all(&pg)
    .await
    .map(Json)
    .map_err(|_| DelegatesErrorResponse::DelegateResponseError)
}

#[utoipa::path(
    get,
    params(
        Date
    ),
    path = "/delegates_at", 
    responses(
        (status = 200, description = "Returned delegates successfully.", body = [Vec<DbDelegate>]), 
        (status = 400, description = "Invalid request", body = [DelegatesErrorResponse]),
        (status = 500, description = "Internal server error", body = [DelegatesErrorResponse])
    )
)]
pub async fn delegates_at(
    // DataserviceDbConnection(con): DataserviceDbConnection,
    Query(date): Query<Date>,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<Delegate>>, DelegatesErrorResponse> {
    delegates_at_date(&pg, &date.at)
        .await
        .map(Json)
        .map_err(|_| DelegatesErrorResponse::DelegateResponseError)
}

pub async fn seats() -> Json<HashMap<String, Vec<u32>>> {
    Json(
        [
            ("XXVII".to_string(), vec![20, 27, 37, 43, 48, 54]),
            ("XXVIII".to_string(), vec![20, 28, 37, 43, 48, 54]),
        ]
        .into_iter()
        .collect(),
    )
}

pub async fn delegates_at_date(pg: &PgPool, date: &NaiveDate) -> sqlx::Result<Vec<Delegate>> {
    sqlx::query_as!(
        Delegate,
        "
        SELECT 
            delegates.id, 
            delegates.name, 
            mandates.party,
            delegates.party as current_party, 
            delegates.image_url, 
            delegates.constituency, 
            CASE 
                WHEN mandates.is_nr THEN 'nr' 
                ELSE ''
            END as council,
            delegates.seat_row, 
            delegates.seat_col, 
            delegates.gender, 
            delegates.is_active, 
            delegates.birthdate, 
            mandates.start_date AS active_since,
            COALESCE(divisions.division_array, '{}') AS divisions
        FROM 
            mandates 
        INNER JOIN 
            delegates ON delegates.id = mandates.delegate_id 
        LEFT JOIN 
            (SELECT 
                delegate_id, 
                ARRAY_AGG(division) AS division_array 
            FROM 
                delegates_divisions 
            GROUP BY 
                delegate_id) AS divisions 
            ON delegates.id = divisions.delegate_id
        WHERE 
            (mandates.is_nr or mandates.is_gov_official)
            --(mandates.name LIKE '%Abgeordnete%' OR mandates.name LIKE '%minister%') 
            and start_date <= $1::date 
            and (case when end_date is null then $1::date else end_date end) >= $1::date;
        ",
        date
    )
    .fetch_all(pg)
    .await
}

#[utoipa::path(
    get,
    path = "/proposals", 
    responses(
        (status = 200, description = "Returned proposals successfully.", body = [Vec<DbProposalQuery>]), 
        (status = 400, description = "Invalid request", body = [DelegatesErrorResponse]),
        (status = 500, description = "Internal server error", body = [DelegatesErrorResponse])
    )
)]
pub async fn proposals(
    PgPoolConnection(con): PgPoolConnection,
) -> Result<Json<Vec<DbProposalQuery>>, DelegatesErrorResponse> {
    get_proposals(&con)
        .await
        .map(Json)
        .map_err(|_| DelegatesErrorResponse::ProposalResponseError)
}
