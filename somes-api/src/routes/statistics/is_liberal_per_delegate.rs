use axum::{debug_handler, Json};

use diesel::sql_types::Bool;
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
pub struct DelegateIsLiberalFilter {
    legis_period: Option<String>,
    party: Option<String>,
    gender: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DelegateIsLiberal {
    delegate_name: String,
    delegate_party: String,
    liberal_score: f64,
}

// #[debug_handler]
pub async fn is_liberal_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DelegateIsLiberalFilter>>,
) -> Result<Json<Vec<DelegateIsLiberal>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = filter.party.with_sql_column("m.party");
    let filter_arg2 = filter.gender.with_sql_column("ds.gender");
    let filter_arg3 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
    let filters = [filter_arg, filter_arg1, filter_arg2, filter_arg3];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let filter = build_filter(&filters);

    let query = format!(
        "
         SELECT * FROM (
    SELECT DISTINCT ON (ds.id) 
        ds.name AS delegate_name,
        COALESCE(ds.party, 'Regierungsmitglied') AS delegate_party,
        (ROUND((pop.is_liberal - pop.is_not_liberal)::numeric, 5)::FLOAT) + 1 AS liberal_score
    FROM 
        political_positions pop
    JOIN 
        delegates ds ON pop.delegate_id = ds.id
    JOIN 
        plenar_speeches ps ON ps.delegate_id = ds.id
    JOIN 
        debates db ON db.id = ps.debate_id
    JOIN 
        plenar_infos pf ON pf.id = db.plenar_id
    JOIN 
        mandates m ON m.delegate_id = ds.id
    WHERE
        {filter}
        AND m.start_date <= (SELECT MIN(add_date) FROM plenar_infos WHERE id = db.plenar_id)
        AND (m.end_date IS NULL OR m.end_date >= (SELECT MAX(add_date) FROM plenar_infos WHERE id = db.plenar_id))
    GROUP BY
        ds.id, ds.name, ds.party, pop.is_liberal, pop.is_not_liberal
    ORDER BY 
        ds.id DESC  
) subquery
ORDER BY liberal_score {desc};
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, DelegateIsLiberal>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
