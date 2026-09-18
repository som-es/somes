use std::sync::Arc;

use axum::{Json, extract::Path};
use combx::api_models::{DbUserMood, MoodBarometer};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use utoipa::ToSchema;
use utoipa_axum::{
    router::{OpenApiRouter, UtoipaMethodRouterExt},
    routes,
};

use crate::{AppState, PgPoolConnection, jwt::Claims, routes::UserError};

pub fn create_proposal_mood_router() -> OpenApiRouter<AppState> {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(8)
            .burst_size(1)
            .finish()
            .unwrap(),
    );
    OpenApiRouter::new()
        .routes(routes!(add_mood_value_route).layer(GovernorLayer::new(governor_conf)))
        .routes(routes!(mood_values_for_gov_prop_route))
        .routes(routes!(user_mood_for_gov_prop_route))
}

#[utoipa::path(
    get,
    path = "/user",
    tag = "gov_proposals",
    params(("gp" = String, Path, description = "Legislative period"), ("inr" = i32, Path, description = "Proposal number")),
    responses(
        (status = 200, description = "User mood for gov prop", body = Option<DbUserMood>),
    )
)]
pub async fn user_mood_for_gov_prop_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Path((gp, inr)): Path<(String, i32)>,
) -> Result<Json<Option<DbUserMood>>, UserError> {
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
    let user_mood = sqlx::query_as!(DbUserMood, "
        select user_mood.id, user_mood, user_id, user_mood.mood_id, created_at, updated_at from user_mood
            join gov_prop_mood gm on gm.mood_id = user_mood.mood_id
        where user_id = $1 and gm.gov_prop_id = $2
    ", claims.id, gov_prop_id).fetch_optional(&pg).await
    .map_err(UserError::SqlFailure)?;
    Ok(Json(user_mood))
}

#[utoipa::path(
    get,
    path = "/",
    tag = "gov_proposals",
    params(("gp" = String, Path, description = "Legislative period"), ("inr" = i32, Path, description = "Proposal number")),
    responses(
        (status = 200, description = "Mood values for gov prop", body = Option<MoodBarometer>),
    )
)]
pub async fn mood_values_for_gov_prop_route(
    PgPoolConnection(pg): PgPoolConnection,
    Path((gp, inr)): Path<(String, i32)>,
) -> Result<Json<Option<MoodBarometer>>, UserError> {
    let barometer = extract_barometer_sqlx(&pg, &gp, inr).await?;
    Ok(Json(barometer))
}

async fn extract_barometer_sqlx(
    pg: &sqlx::Pool<sqlx::Postgres>,
    gp: &str,
    inr: i32,
) -> Result<Option<MoodBarometer>, UserError> {
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
    .fetch_optional(pg)
    .await
    .map_err(UserError::SqlFailure)?;
    Ok(barometer)
}

#[derive(ToSchema, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AddMoodValue {
    pub user_mood: f64,
}

#[utoipa::path(
    post,
    path = "/",
    tag = "gov_proposals",
    params(("gp" = String, Path, description = "Legislative period"), ("inr" = i32, Path, description = "Proposal number")),
    request_body(content = AddMoodValue, content_type = "application/json"),
    responses(
        (status = 200, description = "Add mood value", body = MoodBarometer),
    )
)]
pub async fn add_mood_value_route(
    PgPoolConnection(pg): PgPoolConnection,
    claims: Claims,
    Path((gp, inr)): Path<(String, i32)>,
    Json(add_mood): Json<AddMoodValue>,
) -> Result<Json<MoodBarometer>, UserError> {
    if add_mood.user_mood < -1. || add_mood.user_mood > 1. {
        return Err(UserError::Custom(
            StatusCode::BAD_REQUEST,
            "user mood out of range (must be between -1 and 1".into(),
        ));
    }
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

    Ok(Json(barometer.unwrap()))
}

#[cfg(test)]
#[path = "tests/mood.rs"]
mod tests;
