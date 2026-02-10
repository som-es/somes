use axum::{
    Json, Router, routing::{delete, get, post, put}
};
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{PgPoolConnection, server::AppState};

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct SomesEvent {
    pub id: i32,
    pub title: String,
    pub location: String,
    pub event_date: NaiveDate, 
    pub start_time: NaiveTime,
    pub description: String,
    pub image: Option<String>,
    pub requires_membership: Option<bool>,
    pub requires_registration: Option<bool>,
}

#[derive(Deserialize)]
pub struct EventId {
    pub id: i32,
}


pub async fn create_event_route(
    PgPoolConnection(pg): PgPoolConnection,
    Json(event): Json<SomesEvent>,
) -> crate::Result<Json<()>> {
    create_event_sqlx(&pg, &event)
        .await
        .map(Json)
        .map_err(|e| crate::GenericError::SqlFailure(Some(e)))
}

pub async fn create_event_sqlx(pg: &PgPool, event: &SomesEvent) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO events (
            title, location, event_date, start_time, description, 
            image, requires_membership, requires_registration
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
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
    .execute(pg)
    .await?;

    Ok(())
}

pub async fn delete_event_route(
    PgPoolConnection(pg): PgPoolConnection,
    Json(payload): Json<EventId>,
) -> crate::Result<Json<()>> {
    delete_event_sqlx(&pg, payload.id)
        .await
        .map(Json)
        .map_err(|e| crate::GenericError::SqlFailure(Some(e)))
}

pub async fn delete_event_sqlx(pg: &PgPool, id: i32) -> sqlx::Result<()> {
    sqlx::query!(
        "DELETE FROM events WHERE id = $1",
        id
    )
    .execute(pg)
    .await?;

    Ok(())
}

pub async fn update_event_route(
    PgPoolConnection(pg): PgPoolConnection,
    Json(event): Json<SomesEvent>,
) -> crate::Result<Json<()>> {
    update_event_sqlx(&pg, &event)
        .await
        .map(Json)
        .map_err(|e| crate::GenericError::SqlFailure(Some(e)))
}

pub async fn update_event_sqlx(pg: &PgPool, event: &SomesEvent) -> sqlx::Result<()> {
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