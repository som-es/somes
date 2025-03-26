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
pub struct LegislativeInitiativeFilter {
    legis_period: Option<String>,
    accepted: Option<String>,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LegislativeInitiativeStats {
    total_initiatives: i64,
}

// #[debug_handler]
pub async fn legislative_initiatives_without_simple_majority(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<LegislativeInitiativeFilter>>,
) -> Result<Json<Vec<LegislativeInitiativeStats>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    // Hier baust du deine Filterargumente
    let filter_arg = filter.legis_period.with_sql_column("gp");
    let filter_arg1 = filter.accepted.with_sql_column("accepted");
    let filter_arg2 = Manual("li.requires_simple_majority = false").with_sql_column("");
    let filters = [filter_arg, filter_arg1, filter_arg2];

    // Erstelle den Filterstring
    let filter = build_filter(&filters);

    // Die SQL-Abfrage für die Legislative Initiativen ohne einfache Mehrheit
    let query = format!(
        "
        SELECT
            COUNT(*) AS total_initiatives
        FROM 
            legislative_initiatives li
        WHERE 
            {filter}
        "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, LegislativeInitiativeStats>(&query);

    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
