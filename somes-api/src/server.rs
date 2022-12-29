use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use axum::{
    extract::FromRef,
    http,
    routing::{get, post},
    Router,
};
use somes_common_lib::{SIGNUP_ROUTE, VERIFY_ROUTE, LOGIN_ROUTE};
//use headers::HeaderValue;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    model::NewUser,
    routes::{signup, verify, login},
};

#[derive(Clone)]
struct AppState {
    // that holds some api specific state
    verification_map: VerificationMap,
    redis_con: RedisConn,
}

impl AppState {
    pub async fn new(client: redis::Client) -> AppState {
        AppState { 
            verification_map: Default::default(), 
            redis_con: Arc::new(RwLock::new(client.get_async_connection().await.unwrap()))
        }
    }
}

impl FromRef<AppState> for VerificationMap {
    fn from_ref(app_state: &AppState) -> VerificationMap {
        app_state.verification_map.clone()
    }
}

impl FromRef<AppState> for RedisConn {
    fn from_ref(app_state: &AppState) -> RedisConn {
        app_state.redis_con.clone()
    }
}

pub type VerificationMap = Arc<RwLock<HashMap<String, (NewUser, u64)>>>;
pub type RedisConn = Arc<RwLock<redis::aio::Connection>>;

pub async fn serve(addr: SocketAddr) {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let state = AppState::new(client).await;

    let app = Router::new()
        .route(SIGNUP_ROUTE, post(signup))
        .route(VERIFY_ROUTE, get(verify)) // or post?
        .route(LOGIN_ROUTE, post(login))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([http::Method::GET, http::Method::POST])
                .allow_headers([http::header::CONTENT_TYPE]),
        )
        .with_state(state);
    //.with_state(verification_map)
    //.with_state(verification_hasher);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
