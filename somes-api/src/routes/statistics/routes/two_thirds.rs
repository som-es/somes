use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

use super::error::StatisticsResponse;
use crate::PgPoolConnection;

#[derive(ToSchema, Default, Debug, Deserialize)]
pub struct TwoThirdsFilter {
    legis_period: Option<String>,
}

#[derive(ToSchema, Debug, FromRow, Serialize)]
pub struct TwoThirdsBreakdown {
    category: String,
    positive: i64,
    negative: i64,
    other: i64,
    unknown: i64,
    total: i64,
}

// Count one position per party and initiative, never individual seats.
// Ties are unknown; absence-only records are not a voting position.
const PARTY_QUERY: &str = r#"
WITH positions AS (
    SELECT v.party AS category,
        CASE
            WHEN v.infavor_count > v.against_count AND v.infavor_count > v.abstention_count THEN 'positive'
            WHEN v.against_count > v.infavor_count AND v.against_count > v.abstention_count THEN 'negative'
            WHEN v.abstention_count > v.infavor_count AND v.abstention_count > v.against_count THEN 'other'
            ELSE 'unknown'
        END AS position
    FROM votes v
    JOIN legislative_initiatives li ON li.id = v.legislative_initiatives_id
    WHERE li.requires_simple_majority = false
        AND ($1::text IS NULL OR li.gp = $1)
        AND NULLIF(BTRIM(v.party), '') IS NOT NULL
        AND (v.infavor_count > 0 OR v.against_count > 0 OR v.abstention_count > 0)
)
SELECT category,
    COUNT(*) FILTER (WHERE position = 'positive') AS positive,
    COUNT(*) FILTER (WHERE position = 'negative') AS negative,
    COUNT(*) FILTER (WHERE position = 'other') AS other,
    COUNT(*) FILTER (WHERE position = 'unknown') AS unknown,
    COUNT(*) AS total
FROM positions
GROUP BY category
ORDER BY (COUNT(*) FILTER (WHERE position = 'positive'))::double precision / COUNT(*) DESC, total DESC, category
"#;

const TOPIC_QUERY: &str = r#"
WITH assignments AS (
    SELECT DISTINCT li.id, BTRIM(t.topic) AS category, li.accepted
    FROM legislative_initiatives li
    JOIN topics_legis_init t ON t.legislative_initiatives_id = li.id
    WHERE li.requires_simple_majority = false
        AND ($1::text IS NULL OR li.gp = $1)
        AND NULLIF(BTRIM(t.topic), '') IS NOT NULL
)
SELECT category,
    COUNT(*) FILTER (WHERE accepted = 'a') AS positive,
    COUNT(*) FILTER (WHERE accepted = 'd') AS negative,
    COUNT(*) FILTER (WHERE accepted = 'p') AS other,
    COUNT(*) FILTER (WHERE accepted IS NULL OR accepted NOT IN ('a', 'd', 'p')) AS unknown,
    COUNT(*) AS total
FROM assignments
GROUP BY category
ORDER BY total DESC, category
"#;

#[utoipa::path(
    post,
    path = "/two_thirds_by_party",
    tag = "statistics",
    request_body(content = TwoThirdsFilter, content_type = "application/json"),
    responses((status = 200, description = "Recorded party positions on two-thirds initiatives", body = [TwoThirdsBreakdown]))
)]
pub async fn two_thirds_by_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<TwoThirdsFilter>>,
) -> Result<Json<Vec<TwoThirdsBreakdown>>, StatisticsResponse> {
    sqlx::query_as(PARTY_QUERY)
        .bind(filter.unwrap_or_default().legis_period)
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}

#[utoipa::path(
    post,
    path = "/two_thirds_by_topic",
    tag = "statistics",
    request_body(content = TwoThirdsFilter, content_type = "application/json"),
    responses((status = 200, description = "Two-thirds initiative outcomes by assigned topic", body = [TwoThirdsBreakdown]))
)]
pub async fn two_thirds_by_topic(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<TwoThirdsFilter>>,
) -> Result<Json<Vec<TwoThirdsBreakdown>>, StatisticsResponse> {
    sqlx::query_as(TOPIC_QUERY)
        .bind(filter.unwrap_or_default().legis_period)
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}

#[cfg(test)]
#[path = "tests/two_thirds.rs"]
mod tests;
