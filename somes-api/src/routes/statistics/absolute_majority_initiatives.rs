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
pub struct LegislativeInitiativeFilter {
    legis_period: Option<String>, 
    accepted: Option<String>
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LegislativeInitiativeStats {
    total_initiatives: i64,
}

// #[debug_handler]
pub async fn legislative_initiatives_without_simple_majority(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<LegislativeInitiativeFilter>>,  
) -> Result<Json<LegislativeInitiativeStats>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    // Hier baust du deine Filterargumente
    let filter_arg = filter.legis_period.with_sql_column("li.legislative_period");  
    let filter_arg1 = Some("nr").with_sql_column("council");
    let filters = [filter_arg];

    // Erstelle den Filterstring
    let filter_str = build_filter(&filters);

    // Die SQL-Abfrage für die Legislative Initiativen ohne einfache Mehrheit
    let query = format!(
        "
        SELECT
            COUNT(*) AS total_initiatives
        FROM 
            legislative_initiatives li
        WHERE 
            li.requires_simple_majority = false
        AND
            {filter}
        "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, LegislativeInitiativeStats>(&query);

    // Wende Filter auf die Query an (falls vorhanden)
    filtered_query = bind_values(filtered_query, &filters);

    // Führe die Abfrage aus und gib das Ergebnis zurück
    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
