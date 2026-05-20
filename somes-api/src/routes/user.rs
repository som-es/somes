mod error;
use std::sync::Arc;

pub use error::*;

mod routes;
pub use routes::*;

use axum::{
    routing::{delete, get, post, put},
    Json, Router,
};
use somes_common_lib::{BOOKMARK, LOGIN_ROUTE, RENEW_TOKEN, SEND_MAIL_INFO, TOPIC_SELECTION};
use sqlx::query_as;
use tower_governor::{
    governor::{GovernorConfig, GovernorConfigBuilder},
    GovernorLayer,
};

use crate::{
    jwt::{renew_token_route, Claims},
    model::User,
    server::AppState,
    PgPoolConnection,
};

pub fn create_user_router() -> Router<AppState> {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(4)
            .finish()
            .unwrap(),
    );

    Router::new()
        .route(
            LOGIN_ROUTE,
            post(login).layer(GovernorLayer::new(governor_conf)),
        )
        .route("/delete", delete(delete_account_route))
        .route(RENEW_TOKEN, post(renew_token_route))
        .route(TOPIC_SELECTION, post(add_user_topic_route))
        .route(TOPIC_SELECTION, delete(remove_user_topic_route))
        .route(TOPIC_SELECTION, get(user_topic_selection_route))
        .route(SEND_MAIL_INFO, put(update_send_mail_info_route))
        .route(SEND_MAIL_INFO, get(get_send_mail_info_route))
        .route("/change_email", post(change_mail))
        .route("/verify_email_change", post(verify_email_change))
        .route("/anonymize_email", post(anonymize_email))
        .route("/", get(user_route))
        .nest(BOOKMARK, create_bookmark_router())
}

#[utoipa::path(
    post,
    path = "/user",
    // params(
    //     Claims
    // ),
    responses(
        (status = 200, description = "Returned user successfully.", body = [Vec<User>]),
        // (status = 400, description = "Invalid request", body = [UserError]),
        // (status = 500, description = "Internal server error", body = [UserError])
    )
)]
pub async fn user_route(
    claims: Claims,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<User>, UserError> {
    Ok(query_as!(
        User,
        "select id, email, is_email_hashed, is_admin from somes_user where id = $1",
        claims.id
    )
    .fetch_one(&pg)
    .await
    .map(Json)?)
}

#[cfg(test)]
mod tests {
    use axum::{
        body::{to_bytes, Body},
        http::{Request, StatusCode},
        Router,
    };
    use serde_json::Value;
    use sqlx::postgres::PgPoolOptions;
    use tower::ServiceExt;

    use super::create_user_router;
    use crate::server::AppState;

    fn test_state() -> AppState {
        AppState {
            redis_client: redis::Client::open("redis://127.0.0.1/").unwrap(),
            dataservice_sqlx_pool: PgPoolOptions::new()
                .connect_lazy("postgres://postgres:postgres@127.0.0.1:5432/somes_test")
                .unwrap(),
            meilisearch_client: meilisearch_sdk::client::Client::new(
                "http://127.0.0.1:7700",
                Some("test"),
            )
            .unwrap(),
        }
    }

    #[tokio::test]
    async fn user_route_rejects_requests_without_bearer_token_before_db_access() {
        let app = Router::new()
            .nest("/v1/user", create_user_router())
            .with_state(test_state());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/user/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body["error_type"], "AuthError");
        assert_eq!(body["field"], "MissingToken");
    }
}
