use std::{collections::HashMap, str::FromStr};

use axum::extract::Path;
use axum::routing::get;
use axum::Router;
use axum::{extract::Query, Json};
use chrono::NaiveDate;
use redis::aio::MultiplexedConnection;
use somes_common_lib::FullMandate;
use somes_common_lib::{
    Date, Delegate, DelegateById, InterestShare, LegisPeriod, ALL_ACTIVE, ALL_AT_DATE,
    ALL_AT_DATE_WITH_SEAT_INFO, DELEGATE_QA, EXTEND, GOV_PROPOSALS, ID, SPEECHES_PER_PAGE_ROUTE,
};
use sqlx::PgPool;

use crate::server::AppState;
use crate::{get_json_cache, set_json_cache_with_relevance, PgPoolConnection, RedisConnection};

pub use error::*;
mod absences;
mod ai_chat;
mod delegate_political_position;
mod error;
mod general_delegate_info;
mod gov_officials;
mod interests;
mod left_right_topic_score;
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

pub fn create_gov_officials_router() -> Router<AppState> {
    Router::new()
        .route(ALL_AT_DATE, get(gov_officials_at_date_route))
        .route(GOV_PROPOSALS, get(gov_proposals_by_official))
        .route(EXTEND, get(general_gov_official_info))
}

pub fn create_delegates_router() -> Router<AppState> {
    Router::new()
        .route(ALL_AT_DATE, get(delegates_at))
        .route(ID, get(delegate_by_id_path))
        .route(ALL_ACTIVE, get(delegates))
        .route(DELEGATE_QA, get(delegate_qa))
        .route(SPEECHES_PER_PAGE_ROUTE, get(speeches_by_delegate_per_page))
        .route(
            ALL_AT_DATE_WITH_SEAT_INFO,
            get(delegates_with_seats_near_date_route),
        )
        .route(EXTEND, get(extended_delegate_info))
        .nest("/gov_officials", create_gov_officials_router())
}

#[utoipa::path(
    get,
    params(
        DelegateById
    ),
    path = "/delegate_interests",
    responses(
        (status = 200, description = "Returned delegate interests successfully.", body = [Vec<InterestShare>]),
        // (status = 400, description = "Invalid request", body = [DelegatesErrorResponse]),
        // (status = 500, description = "Internal server error", body = [DelegatesErrorResponse])
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
            * from delegates_with_mandates d
        WHERE
        d.id = $1;
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
        (status = 200, description = "Returned delegate successfully.", body = [Delegate]),
        // (status = 400, description = "Invalid request", body = [DelegatesErrorResponse]),
        // (status = 500, description = "Internal server error", body = [DelegatesErrorResponse])
    )
)]
pub async fn delegate_by_id(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(delegate_by_id): Query<DelegateById>,
) -> Result<Json<Delegate>, DelegatesErrorResponse> {
    delegate_by_id_sqlx(delegate_by_id.delegate_id, &pg, redis_con)
        .await
        .map(Json)
        .map_err(|_| DelegatesErrorResponse::DelegateResponseError)
}

pub async fn delegate_by_id_path(
    RedisConnection(redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Path(id): Path<i32>,
) -> Result<Json<Delegate>, DelegatesErrorResponse> {
    delegate_by_id_sqlx(id, &pg, redis_con)
        .await
        .map(Json)
        .map_err(|_| DelegatesErrorResponse::DelegateResponseError)
}

#[utoipa::path(
    get,
    path = "/delegates",
    responses(
        (status = 200, description = "Returned delegates successfully.", body = [Vec<Delegate>]),
        // (status = 400, description = "Invalid request", body = [DelegatesErrorResponse]),
        // (status = 500, description = "Internal server error", body = [DelegatesErrorResponse])
    )
)]
pub async fn delegates(
    // DataserviceDbConnection(con): DataserviceDbConnection,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<Delegate>>, DelegatesErrorResponse> {
    Ok(sqlx::query_as!(
        Delegate,
        "
SELECT
    * from delegates_with_mandates
WHERE
    is_active
    -- return only delegates with at least on 'is_nr' mandate
    AND EXISTS (
      SELECT 1
      FROM unnest(\"active_mandates: Vec<FullMandate>\") am
      WHERE am.is_nr and am.end_date IS NULL
  );
        "
    )
    .fetch_all(&pg)
    .await
    .map(Json)
    .unwrap())
    // .map_err(|_| DelegatesErrorResponse::DelegateResponseError)
}

#[utoipa::path(
    get,
    params(
        Date
    ),
    path = "/delegates_at",
    responses(
        (status = 200, description = "Returned delegates successfully.", body = [Vec<Delegate>]),
        // (status = 400, description = "Invalid request", body = [DelegatesErrorResponse]),
        // (status = 500, description = "Internal server error", body = [DelegatesErrorResponse])
    )
)]
pub async fn delegates_at(
    // DataserviceDbConnection(con): DataserviceDbConnection,
    RedisConnection(mut redis_con): RedisConnection,
    Query(date): Query<Date>,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<Delegate>>, DelegatesErrorResponse> {
    delegates_at_date(&pg, &date.at, &mut redis_con)
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
    RedisConnection(mut redis_con): RedisConnection,
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

    delegates_with_seats_near_date(&pg, &date.at, &mut redis_con, &gp.period)
        .await
        .map(Json)
        .map_err(|_| DelegatesErrorResponse::DelegateResponseError)
}

pub async fn delegates_with_seats_near_date(
    pg: &PgPool,
    date: &NaiveDate,
    redis_con: &mut MultiplexedConnection,
    gp: &str,
) -> sqlx::Result<Vec<Delegate>> {
    let key = format!("delegates_with_seats_near_date/{date:?}");
    if let Some(delegates) = get_json_cache(redis_con, &key).await {
        return Ok(delegates);
    }

    let delegates = sqlx::query_as!(
        Delegate,
        "
WITH ranked AS (
    SELECT sh.*,
           ROW_NUMBER() OVER (
               PARTITION BY sh.delegate_id
               ORDER BY $1::date - sh.insertion_date::date ASC
           ) AS rn
    FROM seat_history sh
    WHERE sh.gp = $2
      AND $1::date - sh.insertion_date::date >= 0
)

SELECT
    v.id,
    v.name,
    (
          SELECT m.party
          FROM mandates m
          WHERE m.delegate_id = v.id
            AND m.party IS NOT NULL
            AND m.start_date <= $1::date
            AND COALESCE(m.end_date, $1::date) >= $1::date
          ORDER BY m.start_date DESC
          LIMIT 1
        ) AS party,
    v.current_party,
    v.image_url,
    v.constituency,
    'nr' as council,
    r.seat_row,
    r.seat_col,
    v.gender,
    v.is_active,
    v.birthdate,
    v.divisions,
    ARRAY(
                SELECT ROW(start_date, end_date, name, party, is_nr, is_gov_official, is_ministry, is_chancellor, function)::full_mandate
                FROM mandates m
                where delegate_id = v.id and start_date <= $1::date AND COALESCE(end_date, $1::date) >= $1::date
            ) as \"mandates_at_time: Vec<FullMandate>\",
    v.\"active_mandates: Vec<FullMandate>\",
    v.\"mandates: Vec<FullMandate>\"
FROM ranked r
JOIN delegates_with_mandates v ON v.id = r.delegate_id
WHERE r.rn = 1
-- this is already ensured by primary_mandate CTE
 AND EXISTS (
      SELECT 1
      FROM unnest(\"mandates: Vec<FullMandate>\") am
      WHERE am.is_nr
        AND am.start_date <= $1::date
        AND (CASE WHEN am.end_date IS NULL THEN $1::date ELSE am.end_date END) >= $1::date
  );
;
        ",
        date,
        gp
    )
    .fetch_all(pg)
    .await?;

    set_json_cache_with_relevance(redis_con, &key, &delegates, *date).await;

    Ok(delegates)
}

pub async fn gov_officials_at_date_route(
    // DataserviceDbConnection(con): DataserviceDbConnection,
    RedisConnection(mut redis_con): RedisConnection,
    Query(date): Query<Date>,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<Delegate>>, DelegatesErrorResponse> {
    gov_officials_at_date(&pg, &date.at, &mut redis_con)
        .await
        .map(Json)
        .map_err(|_| DelegatesErrorResponse::DelegateResponseError)
}

pub async fn gov_officials_at_date_sqlx(
    pg: &PgPool,
    date: &NaiveDate,
) -> sqlx::Result<Vec<Delegate>> {
    sqlx::query_as!(
        Delegate,
        "
        SELECT
    v.id,
    v.name,
    v.party,
    v.current_party,
    v.image_url,
    v.constituency,
    'gov' as council,
    v.seat_row,
    v.seat_col,
    v.gender,
    v.is_active,
    v.birthdate,
    v.divisions,
    ARRAY(
            SELECT ROW(start_date, end_date, name, party, is_nr, is_gov_official, is_ministry, is_chancellor, function)::full_mandate
            FROM mandates m
            where delegate_id = v.id and start_date <= $1::date AND COALESCE(end_date, $1::date) >= $1::date
        ) as \"mandates_at_time: Vec<FullMandate>\",
    v.\"active_mandates: Vec<FullMandate>\",
    v.\"mandates: Vec<FullMandate>\"
FROM
    delegates_with_mandates v
WHERE
    EXISTS (
        SELECT 1
        FROM unnest(v.\"mandates: Vec<FullMandate>\") am
        WHERE am.is_gov_official
          AND am.start_date <= $1::date
          AND (CASE WHEN am.end_date IS NULL THEN $1::date ELSE am.end_date END) >= $1::date
    );

            ",
        date
    )
    .fetch_all(pg)
    .await
}

pub async fn gov_officials_at_date(
    pg: &PgPool,
    date: &NaiveDate,
    redis_con: &mut MultiplexedConnection,
) -> sqlx::Result<Vec<Delegate>> {
    let key = format!("gov_officials_at_date/{date:?}");
    if let Some(delegates) = get_json_cache(redis_con, &key).await {
        return Ok(delegates);
    }

    let delegates = gov_officials_at_date_sqlx(pg, date).await?;
    set_json_cache_with_relevance(redis_con, &key, &delegates, *date).await;

    Ok(delegates)
}

pub async fn delegates_at_date(
    pg: &PgPool,
    date: &NaiveDate,
    redis_con: &mut MultiplexedConnection,
) -> sqlx::Result<Vec<Delegate>> {
    let key = format!("delegates_at/{date:?}");
    if let Some(delegates) = get_json_cache(redis_con, &key).await {
        return Ok(delegates);
    }

    let delegates = sqlx::query_as!(
        Delegate,
        "
SELECT
    v.id,
    v.name,
    (
      SELECT m.party
      FROM mandates m
      WHERE m.delegate_id = v.id
        AND m.party IS NOT NULL
        AND m.start_date <= $1::date
        AND COALESCE(m.end_date, $1::date) >= $1::date
      ORDER BY m.start_date DESC
      LIMIT 1
    ) AS party,
    v.current_party,
    v.image_url,
    v.constituency,
    'nr' as council,
    v.seat_row,
    v.seat_col,
    v.gender,
    v.is_active,
    v.birthdate,
    v.divisions,
    ARRAY(
        SELECT ROW(start_date, end_date, name, party, is_nr, is_gov_official, is_ministry, is_chancellor, function)::full_mandate
        FROM mandates m
        where delegate_id = v.id and start_date <= $1::date AND COALESCE(end_date, $1::date) >= $1::date
    ) as \"mandates_at_time: Vec<FullMandate>\",
    v.\"active_mandates: Vec<FullMandate>\",
    v.\"mandates: Vec<FullMandate>\"
    from delegates_with_mandates v
WHERE
    EXISTS (
      SELECT 1
      FROM unnest(\"mandates: Vec<FullMandate>\") am
      WHERE am.is_nr
      AND am.start_date <= $1::date
      AND (CASE WHEN am.end_date IS NULL THEN $1::date ELSE am.end_date END) >= $1::date
    );

        ",
        date
    )
    .fetch_all(pg)
    .await?;

    set_json_cache_with_relevance(redis_con, &key, &delegates, *date).await;

    Ok(delegates)
}
