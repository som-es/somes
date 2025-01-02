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
pub struct AgeSpeechComplexityFilter {
    legis_period: Option<String>,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AgeComplexity {
    age_group: String,
    avg_complexity: f64,
}

// #[debug_handler]
pub async fn complexity_at_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<AgeSpeechComplexityFilter>>,
) -> Result<Json<Vec<AgeComplexity>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = Some("nr").with_sql_column("ds.council");
    let filters = [filter_arg, filter_arg1];

    let filter = build_filter(&filters);

    let query = format!(
        " 
        SELECT 
            CASE
                WHEN EXTRACT(YEAR FROM AGE(ds.birthdate)) < 30 THEN 'Under 30'
                WHEN EXTRACT(YEAR FROM AGE(ds.birthdate)) BETWEEN 30 AND 39 THEN '30-39'
                WHEN EXTRACT(YEAR FROM AGE(ds.birthdate)) BETWEEN 40 AND 49 THEN '40-49'
                WHEN EXTRACT(YEAR FROM AGE(ds.birthdate)) BETWEEN 50 AND 59 THEN '50-59'
               ELSE '60+'
        END AS age_group,
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
            age_group
        ORDER BY 
            avg_complexity DESC;
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, AgeComplexity>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
