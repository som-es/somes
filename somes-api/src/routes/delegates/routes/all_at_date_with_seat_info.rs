use crate::routes::DelegateError;
use crate::{get_json_cache, set_json_cache_with_relevance, PgPoolConnection, RedisConnection};
use axum::{extract::Query, Json};
use chrono::NaiveDate;
use redis::aio::MultiplexedConnection;
use somes_common_lib::FullMandate;
use somes_common_lib::{Date, Delegate, LegisPeriod};
use sqlx::PgPool;
use std::str::FromStr;

pub async fn delegates_with_seats_near_date_route(
    RedisConnection(mut redis_con): RedisConnection,
    Query(gp): Query<LegisPeriod>,
    Query(date): Query<Date>,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<Delegate>>, DelegateError> {
    if date.at < NaiveDate::from_str("2024-08-01").map_err(|_| DelegateError::Internal)? {
        return Ok(Json(vec![]));
    }

    Ok(
        delegates_with_seats_near_date(&pg, &date.at, &mut redis_con, &gp.period)
            .await
            .map(Json)?,
    )
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
    v.created_at,
    v.updated_at,
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
