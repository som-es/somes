use axum::{extract::Query, Json};
use chrono::NaiveDate;
use redis::aio::MultiplexedConnection;
use somes_common_lib::FullMandate;
use somes_common_lib::{Date, Delegate};
use sqlx::PgPool;

use crate::routes::DelegateError;
use crate::{get_json_cache, set_json_cache_with_relevance, PgPoolConnection, RedisConnection};

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
pub async fn delegates_at_route(
    // DataserviceDbConnection(con): DataserviceDbConnection,
    RedisConnection(mut redis_con): RedisConnection,
    Query(date): Query<Date>,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<Delegate>>, DelegateError> {
    Ok(delegates_at_date(&pg, &date.at, &mut redis_con)
        .await
        .map(Json)?)
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
    v.created_at,
    v.updated_at,
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
