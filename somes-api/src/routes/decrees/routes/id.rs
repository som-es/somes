use crate::{
    get_json_cache, routes::LegisInitErrorResponse, set_json_cache, PgPoolConnection,
    RedisConnection,
};
use axum::{extract::Query, Json};
use dataservice::combx::OptionalDecree;
use somes_common_lib::{DecreeByRisId, Document};

pub async fn decree_by_ris_id(
    RedisConnection(mut redis_con): RedisConnection,
    PgPoolConnection(pg): PgPoolConnection,
    Query(decree_by_ris_id): Query<DecreeByRisId>,
) -> Result<Json<Option<OptionalDecree>>, LegisInitErrorResponse> {
    let key = format!("decree/{}", &decree_by_ris_id.ris_id);
    if let Some(decree) = get_json_cache::<OptionalDecree>(&mut redis_con, &key).await {
        return Ok(Json(Some(decree)));
    }
    let decree = decree_by_ris_id_sqlx(&pg, &decree_by_ris_id.ris_id).await?;
    set_json_cache(&mut redis_con, &key, &decree).await;
    Ok(Json(decree))
}

async fn decree_by_ris_id_sqlx(
    pg: &sqlx::Pool<sqlx::Postgres>,
    ris_id: &str,
) -> Result<Option<OptionalDecree>, LegisInitErrorResponse> {
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
    .await
    .map_err(|x| {
        LegisInitErrorResponse::GenericErrorResponse(crate::GenericErrorResponse::DbSelectFailure(
            Some(x),
        ))
    })?)
}
