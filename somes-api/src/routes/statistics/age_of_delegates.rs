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
pub struct DelegateAgeFilter {
    legis_period: Option<String>,
    party: Option<String>,
    gender: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DelegateAge {
    delegate_name: String,
    delegate_party: String,
    delegate_age: i32,
}

// #[debug_handler]
pub async fn age_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DelegateAgeFilter>>,
) -> Result<Json<Vec<DelegateAge>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter
        .legis_period
        .with_sql_column("dga.legislative_period");
    let filter_arg1 = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg2 = filter.party.with_sql_column("m.party");
    let filter_arg3 = filter.gender.with_sql_column("ds.gender");
    let filter_arg4 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
    let filter_arg5 = Manual("birthdate is not null").with_sql_column("");
    let filters = [
        filter_arg,
        filter_arg1,
        filter_arg2,
        filter_arg3,
        filter_arg4,
        filter_arg5,
    ];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let filter = build_filter(&filters);

    let query = format!(
        "
SELECT DISTINCT 
    ds.name AS delegate_name, 
    COALESCE(m.party, 'Regierungsmitglied') AS delegate_party, 
    dga.age_at_start as delegate_age
FROM 
    delegates ds
JOIN 
    plenar_speeches ps ON ps.delegate_id = ds.id
JOIN 
    debates db ON db.id = ps.debate_id
JOIN 
    plenar_infos pf ON pf.id = db.plenar_id
JOIN 
    mandates m ON m.delegate_id = ds.id
JOIN 
    delegate_ages dga ON dga.delegate_id = ds.id
WHERE
    {filter}
    AND m.start_date <= pf.add_date
    AND (m.end_date IS NULL OR m.end_date >= pf.add_date)
GROUP BY 
    ds.name, m.party, dga.age_at_start
ORDER BY 
    delegate_age {desc};

    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, DelegateAge>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
