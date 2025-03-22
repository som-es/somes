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
pub struct PartyIsLeftFilter {
    legis_period: Option<String>,
    party: Option<String>,
    gender: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PartyIsLeft {
    delegate_party: String,
    is_left: f64,
    is_not_left: f64,
    is_liberal: f64,
    is_not_liberal: f64,
    neutral_count: f64,
}

// #[debug_handler]
pub async fn is_left_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PartyIsLeftFilter>>,
) -> Result<Json<Vec<PartyIsLeft>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = Manual("m.is_nr").with_sql_column("");
    let filters = [filter_arg, filter_arg1];

    let filter = build_filter(&filters);

    let query = format!(
        "
    SELECT  
       m.party AS delegate_party,
        AVG(pop.is_left) AS is_left,
        AVG(pop.is_not_left) AS is_not_left,
        AVG(pop.is_liberal) AS is_liberal,
        AVG(pop.is_not_liberal) AS is_not_liberal,
        AVG(pop.neutral_count)::FLOAT AS neutral_count
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
        m.party;

    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, PartyIsLeft>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
