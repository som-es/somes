use axum::Json;

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
pub struct DelegateAbsencesFilter {
    legis_period: Option<String>,
    party: Option<String>,
    gender: Option<String>,
    is_desc: bool,
    normalized: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DelegateAbsences {
    delegate_name: String,
    delegate_party: String,
    total_absences: i64,
    total_sessions: i64,
    normalized_absences: f64,
}

// #[debug_handler]
pub async fn absences_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DelegateAbsencesFilter>>,
) -> Result<Json<Vec<DelegateAbsences>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = filter.party.with_sql_column("m.party");
    let filter_arg2 = filter.gender.with_sql_column("ds.gender");
    let filter_arg3 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
    let filters = [filter_arg, filter_arg1, filter_arg2, filter_arg3];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let normalized = if filter.normalized {
        "normalized_absences"
    } else {
        "total_absences"
    };

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
),
session_counts AS (
    SELECT 
        pf.legislative_period, 
        COUNT(DISTINCT pf.id) AS total_sessions
    FROM 
        plenar_infos pf
    JOIN 
        absences ab ON ab.plenary_session_id = pf.id
    GROUP BY 
        pf.legislative_period
)
SELECT 
    ds.name AS delegate_name,
    COALESCE(m.party, 'Regierungsmitglied') AS delegate_party,
    COUNT(DISTINCT ab.id) AS total_absences,
    sc.total_sessions,
    COUNT(DISTINCT ab.id)::FLOAT / NULLIF(sc.total_sessions, 0)::FLOAT AS normalized_absences
FROM
    absences ab
JOIN 
    delegates ds ON ab.delegate_id = ds.id
JOIN 
    mandates m ON m.delegate_id = ds.id
JOIN 
    plenar_infos pf ON pf.id = ab.plenary_session_id
JOIN 
    legislative_period_dates lp ON lp.legislative_period = pf.legislative_period
JOIN 
    session_counts sc ON sc.legislative_period = lp.legislative_period 
WHERE
    {filter}
    AND m.start_date <= lp.end_date
    AND (m.end_date IS NULL OR m.end_date >= lp.start_date)
GROUP BY 
    ds.name, m.party, sc.total_sessions
ORDER BY 
    {normalized} {desc};

    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, DelegateAbsences>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
