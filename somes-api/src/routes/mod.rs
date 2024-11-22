mod delegates;
mod legislative_initiatives;
mod login;
mod parties;
mod questions;
mod reset_password;
mod save_email;
mod signup;
mod statistics;
mod user;
mod verify;
mod walo;

use axum::Json;
pub use delegates::*;
pub use legislative_initiatives::*;
pub use login::*;
pub use parties::*;
pub use questions::*;
use redis::AsyncCommands;
pub use reset_password::*;
pub use save_email::*;
use serde_json::json;
pub use signup::*;
use sqlx::query_as;
pub use statistics::*;
pub use user::*;
pub use verify::*;
pub use walo::*;

use crate::{PgPoolConnection, RedisConnection};

pub async fn all_gps(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<dataservice::with_data::gps::LegislativePeriod>>, Json<serde_json::Value>> {
    Ok(Json(dataservice::with_data::gps::gps(&pg).await.map_err(
        |_| Json(json!({"error": "could not return all legislative periods"})),
    )?))
}

pub async fn topics(
    PgPoolConnection(pg): PgPoolConnection,
    RedisConnection(mut redis_con): RedisConnection,
) -> Result<Json<Vec<Topic>>, Json<serde_json::Value>> {
    return query_as!(
        Topic,
        "select distinct topic from eurovoc_topics_legis_init"
    )
    .fetch_all(&pg)
    .await
    .map(Json)
    .map_err(|_| Json(json!({"error": "redis expire"})));
    

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
}
