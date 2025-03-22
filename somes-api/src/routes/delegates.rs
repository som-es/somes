use std::{collections::HashMap, str::FromStr};

use axum::{extract::Query, Json};
use chrono::NaiveDate;
use dataservice::db::models::DbProposalQuery;
use redis::aio::MultiplexedConnection;
use serde::{Deserialize, Serialize};
use somes_common_lib::{Date, Delegate, DelegateById, DelegateQA, InterestShare, LegisPeriod};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::{
    dataservice::{get_delegate, get_delegates, get_proposals},
    get_json_cache, DataserviceDbConnection, PgPoolConnection, RedisConnection,
};

pub use error::*;
mod absences;
mod ai_chat;
mod delegate_political_position;
mod error;
mod general_delegate_info;
mod gov_officials;
mod interests;
mod named_votes;
mod qa;
mod speeches;
mod stance_topic_score;
pub use absences::*;
pub use ai_chat::*;
pub use delegate_political_position::*;
pub use general_delegate_info::*;
pub use gov_officials::*;
pub use interests::*;
pub use qa::*;
pub use speeches::*;

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

pub async fn delegate_by_id_sqlx(
    delegate_id: i32,
    pg: &PgPool,
    mut redis_con: MultiplexedConnection,
) -> sqlx::Result<Delegate> {
    let key = delegate_id.to_string();
    let res = get_json_cache::<Delegate>(&mut redis_con, &key).await;
    if let Some(res) = res {
        return Ok(res);
    }
    let delegate = sqlx::query_as!(
        Delegate,
        "
        SELECT 
    delegates.id, 
    delegates.name, 
    delegates.party, 
    delegates.party AS current_party, 
    delegates.image_url, 
    delegates.constituency, 
    delegates.council, 
    delegates.seat_row, 
    delegates.seat_col, 
    delegates.gender, 
    delegates.is_active, 
    delegates.birthdate, 
    mandates.name as primary_mandate,
    mandates.start_date AS active_since,
    COALESCE(divisions.division_array, '{}') AS divisions,
    COALESCE(mandate_groups.mandate_array, '{}') AS active_mandates
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
LEFT JOIN 
    (SELECT 
        delegate_id, 
        ARRAY_AGG(name) AS mandate_array 
    FROM 
        mandates
    WHERE 
        end_date IS NULL
    GROUP BY 
        delegate_id) AS mandate_groups
    ON delegates.id = mandate_groups.delegate_id
WHERE 
     delegates.id = $1;

",
        delegate_id
    )
    .fetch_one(pg)
    .await?;

    crate::set_json_cache(&mut redis_con, &key, &delegate)
        .await
        .ok_or(sqlx::Error::WorkerCrashed)?;

    Ok(delegate)
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
    RedisConnection(mut redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id): Query<DelegateById>,
) -> Result<Json<Delegate>, DelegatesErrorResponse> {
    delegate_by_id_sqlx(delegate_by_id.delegate_id, &pg, redis_con)
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
    /*
      sqlx::query_as!(
            Delegate,
            "
    SELECT
        delegates.id,
        delegates.name,
        delegates.party,
        delegates.party AS current_party,
        delegates.image_url,
        delegates.constituency,
        CASE
            WHEN mandates.is_nr THEN 'nr'
            ELSE 'gov'
        END AS council,
        delegates.seat_row,
        delegates.seat_col,
        delegates.gender,
        delegates.is_active,
        delegates.birthdate,
        mandates.start_date AS active_since,
        COALESCE(divisions.division_array, '{}') AS divisions,
        COALESCE(mandate_groups.mandate_array, '{}') AS active_mandates
    FROM
        delegates
    INNER JOIN
        mandates ON delegates.id = mandates.delegate_id
    LEFT JOIN
        (SELECT
            delegate_id,
            ARRAY_AGG(division) AS division_array
        FROM
            delegates_divisions
        GROUP BY
            delegate_id) AS divisions
        ON delegates.id = divisions.delegate_id
    LEFT JOIN
        (SELECT
            delegate_id,
            ARRAY_AGG(CASE WHEN is_gov_official THEN TRUE ELSE FALSE END) AS is_gov_official_array,
            ARRAY_AGG(CASE WHEN is_nr THEN TRUE ELSE FALSE END) AS is_nr_array,
            ARRAY_AGG(name) AS mandate_array
        FROM
            mandates
        WHERE
            (is_gov_official or is_nr)
            AND end_date IS NULL
        GROUP BY
            delegate_id) AS mandate_groups
        ON delegates.id = mandate_groups.delegate_id
    WHERE
        (is_gov_official or is_nr)
        AND mandates.end_date IS NULL
    order by mandates.start_date;
            "
        )
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|_| DelegatesErrorResponse::DelegateResponseError);
    */

    sqlx::query_as!(
        Delegate,
        "
SELECT 
    delegates.id, 
    delegates.name, 
    delegates.party, 
    delegates.party AS current_party, 
    delegates.image_url, 
    delegates.constituency, 
    CASE 
        WHEN mandates.is_nr THEN 'nr' 
        WHEN mandates.is_gov_official THEN 'gov'
        ELSE 'gov'
    END AS council,
    delegates.seat_row, 
    delegates.seat_col, 
    delegates.gender, 
    delegates.is_active, 
    delegates.birthdate, 
    mandates.name as primary_mandate,
    mandates.start_date AS active_since,
    COALESCE(divisions.division_array, '{}') AS divisions,
    COALESCE(mandate_groups.mandate_array, '{}') AS active_mandates
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
LEFT JOIN 
    (SELECT 
        delegate_id, 
        ARRAY_AGG(name) AS mandate_array 
    FROM 
        mandates
    WHERE 
        is_nr 
        AND end_date IS NULL
    GROUP BY 
        delegate_id) AS mandate_groups
    ON delegates.id = mandate_groups.delegate_id
WHERE 
    is_nr 
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

pub async fn delegates_with_seats_near_date_route(
    // DataserviceDbConnection(con): DataserviceDbConnection,
    Query(gp): Query<LegisPeriod>,
    Query(date): Query<Date>,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<Delegate>>, DelegatesErrorResponse> {
    if date.at
        < NaiveDate::from_str("2024-08-01")
            .map_err(|_| DelegatesErrorResponse::DateOutOfRangeError)?
    {
        return Ok(Json(vec![]));
    }

    delegates_with_seats_near_date(&pg, &date.at, &gp.period)
        .await
        .map(Json)
        .map_err(|_| DelegatesErrorResponse::DelegateResponseError)
}

pub async fn delegates_with_seats_near_date(
    pg: &PgPool,
    date: &NaiveDate,
    gp: &str,
) -> sqlx::Result<Vec<Delegate>> {
    sqlx::query_as!(
        Delegate,
        "
SELECT 
    d.id, 
    d.name, 
    m.party,
    d.party AS current_party, 
    d.image_url, 
    d.constituency, 
    CASE 
        WHEN m.is_nr THEN 'nr' 
        WHEN m.is_gov_official THEN 'gov'
        ELSE ''
    END AS council,
    ranked.seat_row, 
    ranked.seat_col, 
    d.gender, 
    d.is_active, 
    d.birthdate, 
    m.start_date AS active_since,
    m.name as primary_mandate,
    COALESCE(divisions.division_array, '{}') AS divisions,
    COALESCE(mandate_groups.mandate_array, '{}') AS active_mandates
FROM (
    SELECT sh.*, 
        ROW_NUMBER() OVER (PARTITION BY sh.delegate_id ORDER BY $1 - sh.insertion_date::date ASC) AS rn
    FROM seat_history AS sh 
    WHERE gp = $2 AND $1 - sh.insertion_date::date >= 0
) AS ranked
JOIN delegates AS d ON d.id = ranked.delegate_id
INNER JOIN mandates AS m ON m.delegate_id = d.id 
LEFT JOIN (
    SELECT 
        delegate_id, 
        ARRAY_AGG(division) AS division_array 
    FROM 
        delegates_divisions 
    GROUP BY 
        delegate_id
) AS divisions ON d.id = divisions.delegate_id
LEFT JOIN (
    SELECT 
        delegate_id, 
        ARRAY_AGG(name) AS mandate_array 
    FROM 
        mandates
    WHERE 
        is_nr 
        AND end_date IS NULL
    GROUP BY 
        delegate_id
) AS mandate_groups ON d.id = mandate_groups.delegate_id
WHERE 
    m.is_nr
    AND m.start_date <= $1::date 
    AND (CASE WHEN m.end_date IS NULL THEN $1::date ELSE m.end_date END) >= $1::date
    AND ranked.rn = 1;

        ", date, gp
    )
    .fetch_all(pg)
    .await
}

pub async fn gov_officials_at_date_route(
    // DataserviceDbConnection(con): DataserviceDbConnection,
    Query(date): Query<Date>,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<Delegate>>, DelegatesErrorResponse> {
    gov_officials_at_date(&pg, &date.at)
        .await
        .map(Json)
        .map_err(|_| DelegatesErrorResponse::DelegateResponseError)
}

pub async fn gov_officials_at_date(pg: &PgPool, date: &NaiveDate) -> sqlx::Result<Vec<Delegate>> {
    sqlx::query_as!(
        Delegate,
        "
    SELECT 
        delegates.id, 
        delegates.name, 
        mandates.party,
        delegates.party AS current_party, 
        delegates.image_url, 
        delegates.constituency, 
        CASE 
            WHEN mandates.is_nr THEN 'nr' 
            WHEN mandates.is_gov_official THEN 'gov'
            ELSE 'gov'
        END AS council,
        delegates.seat_row, 
        delegates.seat_col, 
        delegates.gender, 
        delegates.is_active, 
        delegates.birthdate, 
        mandates.name as primary_mandate,
        mandates.start_date AS active_since,
        COALESCE(divisions.division_array, '{}') AS divisions,
        COALESCE(mandate_groups.mandate_array, '{}') AS active_mandates
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
    LEFT JOIN 
        (SELECT 
            delegate_id, 
            ARRAY_AGG(name) AS mandate_array 
        FROM 
            mandates
        WHERE 
            is_gov_official 
            AND end_date IS NULL
        GROUP BY 
            delegate_id) AS mandate_groups
        ON delegates.id = mandate_groups.delegate_id
    WHERE 
        mandates.is_gov_official
        AND mandates.start_date <= $1::date 
        AND (CASE WHEN mandates.end_date IS NULL THEN $1::date ELSE mandates.end_date END) >= $1::date;

        ",
        date
    )
    .fetch_all(pg)
    .await
}

pub async fn delegates_at_date(pg: &PgPool, date: &NaiveDate) -> sqlx::Result<Vec<Delegate>> {
    sqlx::query_as!(
        Delegate,
        "
    SELECT 
        delegates.id, 
        delegates.name, 
        mandates.party,
        delegates.party AS current_party, 
        delegates.image_url, 
        delegates.constituency, 
        CASE 
            WHEN mandates.is_nr THEN 'nr' 
            WHEN mandates.is_gov_official THEN 'gov'
            ELSE 'gov'
        END AS council,
        delegates.seat_row, 
        delegates.seat_col, 
        delegates.gender, 
        delegates.is_active, 
        delegates.birthdate, 
        mandates.name as primary_mandate,
        mandates.start_date AS active_since,
        COALESCE(divisions.division_array, '{}') AS divisions,
        COALESCE(mandate_groups.mandate_array, '{}') AS active_mandates
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
    LEFT JOIN 
        (SELECT 
            delegate_id, 
            ARRAY_AGG(name) AS mandate_array 
        FROM 
            mandates
        WHERE 
            is_nr 
            AND end_date IS NULL
        GROUP BY 
            delegate_id) AS mandate_groups
        ON delegates.id = mandate_groups.delegate_id
    WHERE 
        mandates.is_nr
        AND mandates.start_date <= $1::date 
        AND (CASE WHEN mandates.end_date IS NULL THEN $1::date ELSE mandates.end_date END) >= $1::date;

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
