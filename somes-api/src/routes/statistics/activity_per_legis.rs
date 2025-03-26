use axum::{debug_handler, Json};

use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres};
use utoipa::ToSchema;

use crate::{
    routes::statistics::{
        error::StatisticsResponse,
        filtering::{bind_values, build_filter, IntoFilterArgument},
    },
    PgPoolConnection,
};

use super::filtering::Manual;

#[derive(ToSchema, Default, Debug, Clone, Serialize, Deserialize)]
pub struct LegisActivityFilter {
    party: Option<String>,
    gender: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LegisActivity {
    legislative_period: String,
    total_absences: i64,
    period_duration_days: f64,
    normalized_absences: f64,
}

// #[debug_handler]
pub async fn activity_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<LegisActivityFilter>>,
) -> Result<Json<Vec<LegisActivity>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.party.with_sql_column("m.party");
    let filter_arg1 = filter.gender.with_sql_column("ds.gender");
    let filter_arg2 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
    let filters = [filter_arg, filter_arg1, filter_arg2];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let filter = build_filter(&filters);

    let query = format!(
        "
       SELECT 
    d.id,
    d.name,
    SUM(
        CASE 
            WHEN p.ityp = 'J' THEN 1 
            WHEN p.ityp = 'AA' THEN 1.2 * proposal_count
            WHEN p.ityp = 'A' THEN 1.2 * proposal_count
            WHEN p.ityp = 'UEA' THEN 1.15 * proposal_count
            WHEN p.ityp = 'I' THEN 1.3 * proposal_count
            ELSE 0
        END
    ) / NULLIF(COALESCE(mandate_duration.mandate_duration_days, 1), 0) AS normalized_activity_score
FROM 
    proposals p
JOIN 
    proposal_delegates pd ON p.id = pd.proposal_id
JOIN 
    delegates d ON pd.delegate_id = d.id
LEFT JOIN (
    SELECT 
        p.id AS proposal_id,
        COUNT(p.id) AS proposal_count
    FROM 
        proposals p
    JOIN 
        proposal_delegates pd ON p.id = pd.proposal_id
    WHERE 
        pd.is_receiver = false
    GROUP BY 
        p.id
) AS proposal_counts ON p.id = proposal_counts.proposal_id
LEFT JOIN (
    SELECT
        m.delegate_id,
        SUM(
            CASE
                WHEN m.end_date IS NULL THEN 
                    (CURRENT_DATE - m.start_date)  
                ELSE 
                    (m.end_date - m.start_date)  
            END
        ) AS mandate_duration_days
    FROM
        mandates m
    WHERE
        m.is_nr = true  
        AND m.is_gov_official = true  
    GROUP BY
        m.delegate_id
) AS mandate_duration ON d.id = mandate_duration.delegate_id
WHERE 
    pd.is_receiver = false
GROUP BY 
    d.id, d.name, mandate_duration.mandate_duration_days
ORDER BY 
    normalized_activity_score DESC;
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, LegisActivity>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
