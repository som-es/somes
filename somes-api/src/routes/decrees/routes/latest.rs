use axum::{extract::Query, Json};
use dataservice::combx::OptionalDecree;
use redis::aio::MultiplexedConnection;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};
use utoipa::ToSchema;
use somes_common_lib::Document;

use crate::{
    routes::FilterError,
    PgPoolConnection, RedisConnection,
};

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize)]
pub struct Days {
    days: u32,
}

pub async fn latest_decrees_route(
    PgPoolConnection(pg): PgPoolConnection,
    Query(days): Query<Days>,
) -> Result<Json<Vec<OptionalDecree>>, FilterError> {
    if days.days > 180 {
        return Err(FilterError::InvalidDays(days.days as u32));
    }

    Ok(
        extract_latest_ministrial_decrees(&pg, days.days as i32)
            .await
            .map(Json)?,
    )
}

pub async fn extract_latest_ministrial_decrees(
    pg: &PgPool,
    days: i32,
) -> sqlx::Result<Vec<OptionalDecree>> {
    let decrees = query_as!(
        OptionalDecree,
        "
        select * from ministrial_decrees_with_docs
        where publication_date > NOW() - make_interval(days => $1)
    order by publication_date desc",
        days
    )
    .fetch_all(pg)
    .await?;
    Ok(decrees)
}
