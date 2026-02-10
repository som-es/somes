use axum::{
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{NaiveDate, NaiveTime};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{PgPoolConnection, jwt::Claims, server::AppState};

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct SomesEvent {
    pub id: Option<i32>,
    pub title: String,
    pub location: String,
    pub event_date: NaiveDate,
    pub start_time: NaiveTime,
    pub description: String,
    pub image: Option<String>,
    pub requires_membership: Option<bool>,
    pub requires_registration: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventId {
    pub id: i32,
}

pub async fn create_event_route(
    claims: Claims,
    PgPoolConnection(pg): PgPoolConnection,
    Json(event): Json<SomesEvent>,
) -> crate::Result<Json<EventId>> {
    if !claims.is_admin {
        return Err(crate::GenericError::Custom((StatusCode::UNAUTHORIZED, "insufficient permissions")))
    }
    create_event_sqlx(&pg, &event)
        .await
        .map(|id| Json(EventId { id }))
        .map_err(|e| crate::GenericError::SqlFailure(Some(e)))
}

pub async fn create_event_sqlx(pg: &PgPool, event: &SomesEvent) -> sqlx::Result<i32> {
    let id = sqlx::query_scalar!(
        r#"
        INSERT INTO events (
            title, location, event_date, start_time, description, 
            image, requires_membership, requires_registration
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#,
        event.title,
        event.location,
        event.event_date,
        event.start_time,
        event.description,
        event.image,
        event.requires_membership,
        event.requires_registration
    )
    .fetch_one(pg)
    .await?;

    Ok(id)
}

pub async fn delete_event_route(
    claims: Claims,
    PgPoolConnection(pg): PgPoolConnection,
    Json(payload): Json<EventId>,
) -> crate::Result<Json<()>> {
    if !claims.is_admin {
        return Err(crate::GenericError::Custom((StatusCode::UNAUTHORIZED, "insufficient permissions")))
    }
    delete_event_sqlx(&pg, payload.id)
        .await
        .map(Json)
        .map_err(|e| crate::GenericError::SqlFailure(Some(e)))
}

pub async fn delete_event_sqlx(pg: &PgPool, id: i32) -> sqlx::Result<()> {
    sqlx::query!("DELETE FROM events WHERE id = $1", id)
        .execute(pg)
        .await?;

    Ok(())
}

pub async fn update_event_route(
    claims: Claims,
    PgPoolConnection(pg): PgPoolConnection,
    Json(event): Json<SomesEvent>,
) -> crate::Result<Json<()>> {
    if !claims.is_admin {
        return Err(crate::GenericError::Custom((StatusCode::UNAUTHORIZED, "insufficient permissions")))
    }
    update_event_sqlx(&pg, &event)
        .await
        .map(Json)
        .map_err(|e| crate::GenericError::SqlFailure(Some(e)))
}

pub async fn update_event_sqlx(pg: &PgPool, event: &SomesEvent) -> sqlx::Result<()> {
    if event.id.is_none() {
        return Err(sqlx::Error::RowNotFound);
    }
    sqlx::query!(
        r#"
        UPDATE events 
        SET title = $1, 
            location = $2, 
            event_date = $3, 
            start_time = $4, 
            description = $5, 
            image = $6, 
            requires_membership = $7, 
            requires_registration = $8
        WHERE id = $9
        "#,
        event.title,
        event.location,
        event.event_date,
        event.start_time,
        event.description,
        event.image,
        event.requires_membership,
        event.requires_registration,
        event.id
    )
    .execute(pg)
    .await?;

    Ok(())
}

pub async fn all_events_route(
    PgPoolConnection(pg): PgPoolConnection,
) -> crate::Result<Json<Vec<SomesEvent>>> {
    all_events_sqlx(&pg)
        .await
        .map(Json)
        .map_err(|e| crate::GenericError::SqlFailure(Some(e)))
}

pub async fn all_events_sqlx(pg: &PgPool) -> sqlx::Result<Vec<SomesEvent>> {
    sqlx::query_as!(
        SomesEvent,
        r#"
        SELECT 
            id, 
            title, 
            location, 
            event_date, 
            start_time, 
            description, 
            image, 
            requires_membership, 
            requires_registration
        FROM events
        "#
    )
    .fetch_all(pg)
    .await
}

pub fn create_events_router() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_event_route))
        .route("/delete", delete(delete_event_route))
        .route("/update", put(update_event_route))
        .route("/", get(all_events_route))
}
