use std::net::SocketAddr;

use axum::{
    extract::FromRef,
    http,
    routing::{get, post},
    Router,
};
use log::{error, info};
use somes_common_lib::{LOGIN_ROUTE, SIGNUP_ROUTE, VERIFY_ROUTE, DELEGATES_ROUTE};
//use headers::HeaderValue;
use crate::{REDIS_DB, routes::delegates};
use tower_http::cors::{Any, CorsLayer};

use crate::routes::{login, signup, verify};

#[derive(Clone)]
pub struct AppState {
    // this holds some api specific state
    //verification_map: VerificationMap,
    pub redis_client: redis::Client,
}

impl AppState {
    pub async fn new(client: redis::Client) -> AppState {
        AppState {
            //      verification_map: Default::default(),
            //redis_client: Arc::new(RwLock::new(client)),
            redis_client: client,
        }
    }
}
/*
impl FromRef<AppState> for VerificationMap {
    fn from_ref(app_state: &AppState) -> VerificationMap {
        app_state.verification_map.clone()
    }
}*/

/*impl FromRef<AppState> for RedisClient {
    fn from_ref(app_state: &AppState) -> RedisClient {
        app_state.redis_client.clone()
    }
}*/

impl FromRef<AppState> for redis::Client {
    fn from_ref(app_state: &AppState) -> redis::Client {
        app_state.redis_client.clone()
    }
}

//pub type RedisClient = Arc<RwLock<redis::Client>>;

pub async fn serve(addr: SocketAddr) {
    let Ok(client) = redis::Client::open("redis://127.0.0.1/") else {
        error!("Could not establish redis database connection!");
        return;
    };

    info!("Established redis database connection to {REDIS_DB}.");

    let state = AppState::new(client).await;

    let app = Router::new()
        .route(SIGNUP_ROUTE, post(signup))
        .route(VERIFY_ROUTE, get(verify)) // or post?
        .route(LOGIN_ROUTE, post(login))
        .route(DELEGATES_ROUTE, get(delegates))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([http::Method::GET, http::Method::POST])
                .allow_headers([http::header::CONTENT_TYPE]),
        )
        .with_state(state);
    //.with_state(verification_map)
    //.with_state(verification_hasher);

    let server = match axum::Server::try_bind(&addr) {
        Ok(server) => server,
        Err(e) => panic!("Could not initialize API: {e}"),
    };

    info!("Initialized API");

    if let Err(e) = server.serve(app.into_make_service()).await {
        error!("API returned error state: {e}")
    }
}
