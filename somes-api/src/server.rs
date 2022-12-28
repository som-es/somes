use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use axum::{
    extract::FromRef,
    http,
    routing::{get, post},
    Router,
};
use somes_common_lib::SIGNUP_ROUTE;
//use headers::HeaderValue;
use tower_http::cors::{Any, CorsLayer};

use crate::{model::NewUser, routes::signup};

#[derive(Clone, Default)]
struct AppState {
    // that holds some api specific state
    verification_map: VerificationMap,
    verification_hasher: VerificationHasher,
}

impl FromRef<AppState> for VerificationMap {
    fn from_ref(app_state: &AppState) -> VerificationMap {
        app_state.verification_map.clone()
    }
}

impl FromRef<AppState> for VerificationHasher {
    fn from_ref(app_state: &AppState) -> VerificationHasher {
        app_state.verification_hasher.clone()
    }
}

pub type VerificationMap = Arc<RwLock<HashMap<u64, (NewUser, u64)>>>;
pub type VerificationHasher = Arc<RwLock<DefaultHasher>>;

pub async fn serve(addr: SocketAddr) {
    let state = AppState::default();

    let app = Router::new()
        .route(SIGNUP_ROUTE, post(signup))
        //.route("/auth", post(authorize))
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
