use std::net::SocketAddr;

use axum::{
    extract::FromRef,
    http,
    routing::{get, post},
    Router,
};
// use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use log::{error, info};
use somes_common_lib::{
    CALL_TO_ORDERS_PER_PARTY_DELEGATES, DELEGATES_BY_CALL_TO_ORDERS, DELEGATES_ROUTE,
    LATEST_LEGIS_INITS_ROUTE, LATEST_VOTE_RESULTS_ROUTE, LEGIS_INIT_ROUTE, LOGIN_ROUTE,
    PROPOSALS_ROUTE, SIGNUP_ROUTE, SPEAKERS_BY_HOURS, VERIFY_ROUTE,
};
//use headers::HeaderValue;
use crate::{
    routes::{
        call_to_orders_per_party_delegates, delegate_by_call_to_orders, delegates,
        latest_legis_inits, latest_vote_results, legis_inits, proposals, save_email,
        speakers_by_hours,
    },
    DATABASE_URL, DATASERVICE_URL, REDIS_DB,
};
use tower_http::cors::{Any, CorsLayer};

use crate::routes::{login, signup, verify};

#[derive(Clone)]
pub struct AppState {
    // this holds some api specific state
    //verification_map: VerificationMap,
    pub redis_client: redis::Client,
    pub somes_db_pool: deadpool_diesel::postgres::Pool,
    pub dataservice_db_pool: deadpool_diesel::postgres::Pool,
}

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

impl AppState {
    pub async fn new(
        redis_client: redis::Client,
        somes_db_pool: deadpool_diesel::postgres::Pool,
        dataservice_db_pool: deadpool_diesel::postgres::Pool,
    ) -> AppState {
        AppState {
            //      verification_map: Default::default(),
            //redis_client: Arc::new(RwLock::new(client)),
            redis_client,
            somes_db_pool,
            dataservice_db_pool,
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
    let Ok(client) = redis::Client::open(REDIS_DB) else {
        error!("Could not establish redis database connection!");
        return;
    };

    info!("Established redis database connection to {REDIS_DB}.");

    /*
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(DATABASE_URL);
    let postgres_pool = bb8::Pool::builder().build(config).await.unwrap();
     */

    let somes_db_manager =
        deadpool_diesel::postgres::Manager::new(DATABASE_URL, deadpool_diesel::Runtime::Tokio1);
    let somes_db_pool = deadpool_diesel::postgres::Pool::builder(somes_db_manager)
        .build()
        .unwrap();

    let dataservice_db_manager =
        deadpool_diesel::postgres::Manager::new(DATASERVICE_URL, deadpool_diesel::Runtime::Tokio1);

    let dataservice_db_pool = deadpool_diesel::postgres::Pool::builder(dataservice_db_manager)
        .build()
        .unwrap();

    let state = AppState::new(client, somes_db_pool, dataservice_db_pool).await;

    let app = Router::new()
        .route(SIGNUP_ROUTE, post(signup))
        .route(VERIFY_ROUTE, get(verify)) // or post?
        .route(LOGIN_ROUTE, post(login))
        .route(DELEGATES_ROUTE, get(delegates))
        .route(PROPOSALS_ROUTE, get(proposals))
        .route(LEGIS_INIT_ROUTE, post(legis_inits))
        .route(LATEST_LEGIS_INITS_ROUTE, get(latest_legis_inits))
        .route(LATEST_VOTE_RESULTS_ROUTE, get(latest_vote_results))
        // statistics
        .route(SPEAKERS_BY_HOURS, get(speakers_by_hours))
        .route(DELEGATES_BY_CALL_TO_ORDERS, get(delegate_by_call_to_orders))
        .route(
            CALL_TO_ORDERS_PER_PARTY_DELEGATES,
            get(call_to_orders_per_party_delegates),
        )
        .route("/save_email", post(save_email))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([http::Method::GET, http::Method::POST])
                .allow_headers([http::header::CONTENT_TYPE]),
        )
        .with_state(state);
    //.with_state(verification_map)
    //.with_state(verification_hasher);

    info!("Binding API on {addr}");

    let server = match axum::Server::try_bind(&addr) {
        Ok(server) => server,
        Err(e) => panic!("Could not initialize API: {e}"),
    };

    info!("Initialized API");

    if let Err(e) = server.serve(app.into_make_service()).await {
        error!("API returned error state: {e}")
    }
}
