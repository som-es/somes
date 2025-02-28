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
pub struct LegisAbsencesFilter {
    party: Option<String>,
    gender: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LegisAbsences {
    legislative_period: String,
    total_absences: i64,
    period_duration_days: f64,
    normalized_absences: f64,

}

// #[debug_handler]
pub async fn absences_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<LegisAbsencesFilter>>,
) -> Result<Json<Vec<LegisAbsences>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.party.with_sql_column("m.party");
    let filter_arg1 = filter.gender.with_sql_column("ds.gender");
    let filter_arg2 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
    let filters = [filter_arg, filter_arg1, filter_arg2];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let filter = build_filter(&filters);

    let query = format!(
        "
        WITH legislative_period_dates AS (
    SELECT 
        legislative_period, 
        MIN(add_date) AS start_date, 
        MAX(add_date) AS end_date
    FROM 
        plenar_infos
    GROUP BY 
        legislative_period
)
SELECT 
    pf.legislative_period AS legislative_period,
    COUNT(DISTINCT ab.id) AS total_absences,
    EXTRACT(DAY FROM (ld.end_date - ld.start_date))::FLOAT AS period_duration_days,
    COUNT(DISTINCT ab.id)::FLOAT / NULLIF(EXTRACT(DAY FROM (ld.end_date - ld.start_date)), 0)::FLOAT AS normalized_absences
FROM 
    absences ab
JOIN 
    delegates ds ON ab.delegate_id = ds.id
JOIN 
    mandates m ON m.delegate_id = ds.id
JOIN 
    plenar_infos pf ON pf.id = ab.plenary_session_id
JOIN 
    legislative_period_dates ld ON pf.legislative_period = ld.legislative_period
WHERE
    {filter}
    AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = pf.id)
    AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = pf.id))
GROUP BY 
    pf.legislative_period, ld.start_date, ld.end_date
ORDER BY 
    normalized_absences {desc};

    "
    );


    let mut filtered_query = sqlx::query_as::<Postgres, LegisAbsences>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
