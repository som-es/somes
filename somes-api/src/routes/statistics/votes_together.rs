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
pub struct VotesTogetherFilter {
    legis_period: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct VotesTogether {
    party_1: String,
    party_2: String,
    same_votes: i64,
}

// #[debug_handler]
pub async fn votes_together(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<VotesTogetherFilter>>,
) -> Result<Json<Vec<VotesTogether>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();

    let filter_arg = filter.legis_period.with_sql_column("gp");
    let filters = [filter_arg];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let filter = build_filter(&filters);

    let query = format!(
        "
        
WITH paired_votes AS (
    SELECT 
        v1.party AS party_1,
        v2.party AS party_2,
        COUNT(*) AS same_votes
    FROM 
        votes v1
    JOIN 
        votes v2
    ON 
        v1.legislative_initiatives_id = v2.legislative_initiatives_id
        AND v1.infavor = v2.infavor
        AND v1.party < v2.party  
    JOIN 
        legislative_initiatives li
    ON 
        v1.legislative_initiatives_id = li.id
    WHERE 
        {filter}
	 
    GROUP BY 
        v1.party, v2.party
)
SELECT 
    party_1,
    party_2,
    same_votes
FROM 
    paired_votes
ORDER BY 
    same_votes {desc};
        "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, VotesTogether>(&query);

    filtered_query = bind_values(filtered_query, &filters);

    filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
}
