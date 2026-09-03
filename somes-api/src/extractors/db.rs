use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use reqwest::StatusCode;
use sqlx::PgPool;

use crate::AppState;

pub struct McpRedisConnection(pub redis::aio::ConnectionManager);

impl FromRequestParts<AppState> for McpRedisConnection {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let con = state.mcp_redis.connection.clone();

        Ok(Self(con))
    }
}

pub struct RedisConnection(pub redis::aio::ConnectionManager);

impl FromRequestParts<AppState> for RedisConnection {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let parliament = parts
            .extensions
            .get::<combx::Parliament>()
            .copied()
            .unwrap_or_default();

        let conn = state.redis(parliament);
        Ok(Self(conn))
    }
}

impl FromRef<AppState> for redis::Client {
    fn from_ref(app_state: &AppState) -> redis::Client {
        app_state.redis.client.clone()
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(app_state: &AppState) -> PgPool {
        app_state.dataservice_sqlx_pool.clone()
    }
}

// Parliament-aware: `/api/eu/...` requests carry a `Parliament::Eu` extension
// (injected by the route nest in `crate::server`) and get the EU Postgres pool;
// everything else falls back to the Austrian pool.
impl FromRequestParts<AppState> for PgPoolConnection {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let parliament = parts
            .extensions
            .get::<combx::Parliament>()
            .copied()
            .unwrap_or_default();
        Ok(Self(state.pool(parliament)))
    }
}

pub struct PgPoolConnection(pub PgPool);

pub struct AtPgPoolConnection(pub PgPool);

impl FromRequestParts<AppState> for AtPgPoolConnection {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(state.dataservice_sqlx_pool.clone()))
    }
}
