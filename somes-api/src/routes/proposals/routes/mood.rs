use axum::{
    Json, Router,
    extract::Path,
    routing::{get, post},
};
use combx::api_models::MoodBarometer;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{AppState, PgPoolConnection, jwt::Claims, routes::UserError};

pub fn create_proposal_mood_router() -> Router<AppState> {
    Router::new()
        .route("/", post(add_mood_value_route))
        .route("/", get(add_mood_value_route))
}

pub async fn mood_values_for_gov_prop(
    PgPoolConnection(pg): PgPoolConnection,
    Path((gp, inr)): Path<(String, i32)>,
) -> Result<Json<MoodBarometer>, UserError> {
    let barometer = extract_barometer_sqlx(&pg, &gp, inr).await?;
    Ok(Json(barometer))
}

async fn extract_barometer_sqlx(
    pg: &sqlx::Pool<sqlx::Postgres>,
    gp: &str,
    inr: i32,
) -> Result<MoodBarometer, UserError> {
    let barometer = sqlx::query_as!(
        MoodBarometer,
        r#"
            select
                mp.id as gov_prop_id,
                m.id as mood_id,
                m.auto_mood,
                m.pre_aggregated_user_mood,
                coalesce(
                    array_agg(um.user_mood) filter (where um.user_mood is not null),
                    '{}'
                ) as "user_moods!: Vec<f64>"
            from ministrial_proposals as mp
            join gov_prop_mood as gpm on gpm.gov_prop_id = mp.id
            join mood as m on m.id = gpm.mood_id
            left join user_mood as um on um.mood_id = m.id
            where mp.gp = $1 and mp.inr = $2
            group by mp.id, m.id, m.auto_mood, m.pre_aggregated_user_mood
        "#,
        gp,
        inr
    )
    .fetch_one(pg)
    .await
    .map_err(UserError::SqlFailure)?;
    Ok(barometer)
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AddMoodValue {
    pub user_mood: f64,
}

pub async fn add_mood_value_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Path((gp, inr)): Path<(String, i32)>,
    Json(add_mood): Json<AddMoodValue>,
) -> Result<Json<MoodBarometer>, UserError> {
    let gov_prop_id: Option<i32> = sqlx::query_scalar!(
        "select id from ministrial_proposals where gp = $1 and inr = $2",
        gp,
        inr
    )
    .fetch_optional(&pg)
    .await
    .map_err(UserError::SqlFailure)?;
    let gov_prop_id = gov_prop_id.ok_or(UserError::Custom(
        StatusCode::NOT_FOUND,
        "gov proposal not found".into(),
    ))?;

    let mut tx = pg.begin().await.map_err(UserError::SqlFailure)?;

    sqlx::query!("select pg_advisory_xact_lock($1)", gov_prop_id as i64)
        .execute(&mut *tx)
        .await
        .map_err(UserError::SqlFailure)?;

    let mood_id: i64 = match sqlx::query_scalar!(
        "select mood_id from gov_prop_mood where gov_prop_id = $1",
        gov_prop_id
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(UserError::SqlFailure)?
    {
        Some(mood_id) => mood_id,
        None => {
            let mood_id: i64 =
                sqlx::query_scalar!("insert into mood (auto_mood) values (0) returning id")
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(UserError::SqlFailure)?;
            sqlx::query!(
                "insert into gov_prop_mood (gov_prop_id, mood_id) values ($1, $2)",
                gov_prop_id,
                mood_id
            )
            .execute(&mut *tx)
            .await
            .map_err(UserError::SqlFailure)?;
            mood_id
        }
    };

    sqlx::query!(
        "insert into user_mood (user_mood, user_id, mood_id, updated_at)
         values ($1, $2, $3, now())
         on conflict (user_id, mood_id)
         do update set user_mood = excluded.user_mood, updated_at = now()",
        add_mood.user_mood,
        claims.id,
        mood_id
    )
    .execute(&mut *tx)
    .await
    .map_err(UserError::SqlFailure)?;

    sqlx::query!(
        "update mood
         set pre_aggregated_user_mood = (select avg(user_mood) from user_mood where mood_id = $1)
         where id = $1",
        mood_id
    )
    .execute(&mut *tx)
    .await
    .map_err(UserError::SqlFailure)?;

    tx.commit().await.map_err(UserError::SqlFailure)?;

    let barometer = extract_barometer_sqlx(&pg, &gp, inr).await?;

    Ok(Json(barometer))
}

#[cfg(test)]
#[path = "tests/mood.rs"]
mod tests;
