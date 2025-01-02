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

#[derive(ToSchema, Default, Debug, Clone, Serialize, Deserialize)]
pub struct GenderSpeechComplexityFilter {
    legis_period: Option<String>,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct GenderComplexity {
    gender: String,
    avg_complexity: f64,
}

// #[debug_handler]
pub async fn complexity_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<GenderSpeechComplexityFilter>>,
) -> Result<Json<Vec<GenderComplexity>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = Some("nr").with_sql_column("ds.council");
    let filters = [filter_arg, filter_arg1];

    let filter = build_filter(&filters);

    let query = format!(
        " 
        SELECT 
            ds.gender AS gender,
            AVG((sc.flesch_kincaid + sc.smog + sc.gunning_fog + sc.coleman_liau) / 4) AS avg_complexity
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
        WHERE 
            {filter}  
        GROUP BY 
            ds.gender
        ORDER BY 
            avg_complexity DESC;
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, GenderComplexity>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
