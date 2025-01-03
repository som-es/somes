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
pub struct PartyAgeFilter {
    legis_period: Option<String>,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PartyAge {
    delegate_party: String,
    average_age: f64,
}

// #[debug_handler]
pub async fn age_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PartyAgeFilter>>,
) -> Result<Json<Vec<PartyAge>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
    let filter_arg1 = Some("nr").with_sql_column("ds.council");
    let filters = [filter_arg, filter_arg1];

    let filter = build_filter(&filters);

    let query = format!(
        "
        SELECT 
            ds.party AS delegate_party,
            AVG(
            (EXTRACT(YEAR FROM AGE(birthdate)) + 
            (EXTRACT(MONTH FROM AGE(birthdate)) / 12.0) + 
            (EXTRACT(DAY FROM AGE(birthdate)) / 365.25)))::FLOAT as daverage_age     
        FROM 
            delegates ds
        JOIN 
            plenar_speeches ps ON ps.delegate_id = ds.id
        JOIN 
            debates db ON db.id = ps.debate_id
        JOIN 
            plenar_infos pf ON pf.id = db.plenar_id
        WHERE 
            {filter}
        GROUP BY 
            ds.party
        ORDER BY 
            average_age DESC;
    "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, PartyAge>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
