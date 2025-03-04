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
pub struct PartySpeechComplexityFilter {
    legis_period: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PartyComplexity {
    delegate_party: String,
    avg_complexity: f64,
}

// #[debug_handler]
pub async fn complexity_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PartySpeechComplexityFilter>>,
) -> Result<Json<Vec<PartyComplexity>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = Manual("m.is_nr").with_sql_column("");
    let filters = [filter_arg, filter_arg1];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let filter = build_filter(&filters);

    let query = format!(
        " 
        SELECT 
             m.party AS delegate_party,
             AVG((COALESCE(sc.flesch_kincaid, 0) + COALESCE(sc.smog, 0) + COALESCE(sc.gunning_fog, 0) + COALESCE(sc.coleman_liau, 0)) / 4) AS avg_complexity
         FROM 
            speech_complexity sc
        JOIN 
            plenar_speeches ps ON ps.id = sc.speech_id
        JOIN 
            delegates ds ON ps.delegate_id = ds.id
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
    m.party
ORDER BY 
    avg_complexity {desc};
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, PartyComplexity>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
