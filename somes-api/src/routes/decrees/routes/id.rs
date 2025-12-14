use crate::{
    get_json_cache, routes::FilterError, set_json_cache, PgPoolConnection, RedisConnection,
};
use axum::{extract::Path, Json};
use dataservice::combx::OptionalDecree;
use somes_common_lib::Document;

pub async fn decree_by_ris_id_route(
    RedisConnection(mut redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Path(decree_ris_id): Path<String>,
) -> Result<Json<OptionalDecree>, FilterError> {
    let key = format!("decree/{}", &decree_ris_id);
    if let Some(decree) = get_json_cache::<OptionalDecree>(&mut redis_con, &key).await {
        return Ok(Json(decree));
    }
    let decree = decree_by_ris_id_sqlx(&pg, &decree_ris_id)
        .await?
        .ok_or(FilterError::NotFound)?;
    set_json_cache(&mut redis_con, &key, &decree).await;
    Ok(Json(decree))
}

async fn decree_by_ris_id_sqlx(
    pg: &sqlx::Pool<sqlx::Postgres>,
    ris_id: &str,
) -> sqlx::Result<Option<OptionalDecree>> {
    Ok(sqlx::query_as!(
        OptionalDecree,
        r#"
            select * from ministrial_decrees_with_docs
            WHERE
                ris_id = $1
            "#,
        ris_id
    )
    .fetch_optional(pg)
    .await?)
}
