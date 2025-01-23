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
pub struct DelegateAbsencesFilter {
    legis_period: Option<String>,
    party: Option<String>,
    gender: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DelegateAbsences {
    delegate_name: String,
    delegate_party: String,
    total_absences: i64,
}

// #[debug_handler]
pub async fn absences_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DelegateAbsencesFilter>>,
) -> Result<Json<Vec<DelegateAbsences>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = filter.party.with_sql_column("ds.party");
    let filter_arg2 = filter.gender.with_sql_column("ds.gender");
    let filter_arg3 = Manual("m.is_nr").with_sql_column("");
    let filters = [filter_arg, filter_arg1, filter_arg2, filter_arg3];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let filter = build_filter(&filters);

    let query = format!(
        "
        SELECT 
    ds.name AS delegate_name,
    m.party AS delegate_party,
    COUNT(DISTINCT ab.id) AS total_absences
FROM 
    absences ab
JOIN 
    delegates ds ON ab.delegate_id = ds.id
JOIN 
    mandates m ON m.delegate_id = ds.id
JOIN 
    plenar_infos pf ON pf.id = ab.plenary_session_id
WHERE
    {filter}
    AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = pf.id)
    AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = pf.id))
GROUP BY 
    ds.name, m.party
ORDER BY 
    total_absences {desc};

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
