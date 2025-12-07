use axum::Json;
use serde_json::json;
use sqlx::query_as;

use crate::{jwt::Claims, PgPoolConnection};

use super::UniqueTopic;

pub async fn eurovoc_topics(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<UniqueTopic>>, Json<serde_json::Value>> {
    return query_as!(
        UniqueTopic,
        "select id, topic_name as topic from unique_eurovoc_topics order by topic"
    )
    .fetch_all(&pg)
    .await
    .map(Json)
    .map_err(|_| Json(json!({"error": "redis expire"})));
    /*

    match redis_con.get::<_, Vec<String>>("topics").await {
        Ok(v) => {
            if v.is_empty() {
                let topics = query_as!(
                    Topic,
                    "select distinct topic from eurovoc_topics_legis_init"
                )
                .fetch_all(&pg)
                .await;
                match topics {
                    Ok(v) => {
                        redis_con
                            .set::<_, _, ()>(
                                "topics",
                                v.clone().into_iter().map(|x| x.topic).collect::<Vec<_>>(),
                            )
                            .await.unwrap();
                            // .map_err(|_| Json(json!({"error": "redis"})))?;
                        redis_con
                            .expire::<_, ()>("topics", 60 * 15)
                            .await
                            .map_err(|_| Json(json!({"error": "redis expire"})))?;

                        return Ok(Json(v));
                    }
                    Err(_) => return Err(Json(json!({"error": "db error"}))),
                }
            }
            return Ok(Json(
                v.into_iter()
                    .map(|x| Topic { topic: x })
                    .collect::<Vec<_>>(),
            ));
        }
        Err(_) => return Err(Json(json!({"error": "db error"}))),
    }
    */
}

pub async fn topics(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<UniqueTopic>>, Json<serde_json::Value>> {
    return query_as!(
        UniqueTopic,
        "select id, topic_name as topic from unique_topics order by topic"
    )
    .fetch_all(&pg)
    .await
    .map(Json)
    .map_err(|_| Json(json!({"error": "redis expire"})));
}
