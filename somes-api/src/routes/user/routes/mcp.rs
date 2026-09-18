use axum::Json;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngCore;
use redis::AsyncTypedCommands;
use sha3::{Digest, Sha3_256};
use somes_common_lib::{HasMcpToken, JWTInfo as McpToken};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{AppState, GenericError, McpRedisConnection, jwt::Claims};

fn generate_mcp_token() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);

    format!("mcp_{}", URL_SAFE_NO_PAD.encode(bytes))
}

async fn revoke_mcp_token(
    redis: &mut (impl redis::aio::ConnectionLike + Send + Sync),
    user_id: i32,
) -> crate::Result<()> {
    let token = redis
        .get(user_id)
        .await
        .map_err(|e| GenericError::RedisFailure(e))?;

    if let Some(token) = token {
        redis
            .unlink(token)
            .await
            .map_err(|e| GenericError::RedisFailure(e))?;
    }

    redis
        .unlink(user_id)
        .await
        .map_err(|e| GenericError::RedisFailure(e))?;
    Ok(())
}

async fn generate_mcp_token_pipeline(
    redis: &mut (impl redis::aio::ConnectionLike + Send + Sync),
    user_id: i32,
) -> crate::Result<String> {
    let token = generate_mcp_token();

    let mut hasher = Sha3_256::new();
    hasher.update(token.as_bytes());
    let hash = hasher.finalize();
    let hashed_token = hex::encode(hash);

    redis
        .set(user_id, &hashed_token)
        .await
        .map_err(|e| GenericError::RedisFailure(e))?;
    redis
        .set(&hashed_token, user_id)
        .await
        .map_err(|e| GenericError::RedisFailure(e))?;
    Ok(token)
}

pub fn create_user_mcp_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(has_mcp_token_route))
        .routes(routes!(revoke_mcp_token_route))
        .routes(routes!(create_mcp_token_route))
}

#[utoipa::path(
    get,
    path = "/mcp",
    tag = "user",
    responses(
        (status = 200, description = "Has mcp token", body = HasMcpToken),
    )
)]
pub async fn has_mcp_token_route(
    claims: Claims,
    McpRedisConnection(mut redis_con): McpRedisConnection,
) -> crate::Result<Json<HasMcpToken>> {
    let res = redis_con
        .get(claims.id)
        .await
        .map_err(|e| GenericError::RedisFailure(e))?;
    Ok(Json(HasMcpToken {
        has_token: res.is_some(),
    }))
}

#[utoipa::path(
    delete,
    path = "/mcp",
    tag = "user",
    responses(
        (status = 200, description = "Revoke mcp token"),
    )
)]
pub async fn revoke_mcp_token_route(
    claims: Claims,
    McpRedisConnection(mut redis_con): McpRedisConnection,
) -> crate::Result<Json<()>> {
    revoke_mcp_token(&mut redis_con, claims.id).await?;
    Ok(Json(()))
}

#[utoipa::path(
    post,
    path = "/mcp",
    tag = "user",
    responses(
        (status = 200, description = "Create mcp token", body = McpToken),
    )
)]
pub async fn create_mcp_token_route(
    claims: Claims,
    McpRedisConnection(mut redis_con): McpRedisConnection,
) -> crate::Result<Json<McpToken>> {
    revoke_mcp_token(&mut redis_con, claims.id).await?;
    let token = generate_mcp_token_pipeline(&mut redis_con, claims.id).await?;
    Ok(Json(McpToken {
        access_token: token,
    }))
}
