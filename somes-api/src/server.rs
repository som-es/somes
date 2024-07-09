use std::{net::SocketAddr, path::PathBuf};

use axum::{
    extract::FromRef,
    http,
    routing::{get, post},
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use dataservice::db::models::{DbLegislativeInitiativeQuery, DbParty};
// use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use log::{error, info};
use somes_common_lib::{
    CALL_TO_ORDERS_PER_PARTY_DELEGATES, DELEGATES_BY_CALL_TO_ORDERS,
    DELEGATES_BY_CALL_TO_ORDERS_AND_LEGIS_PERIOD, DELEGATES_ROUTE, LATEST_LEGIS_INITS_ROUTE,
    LATEST_VOTE_RESULTS_ROUTE, LEGIS_INIT_ROUTE, LOGIN_ROUTE, PARTIES, PROPOSALS_ROUTE,
    SIGNUP_ROUTE, SPEAKERS_BY_HOURS, SPEAKERS_BY_HOURS_AND_LEGIS_PERIOD, USER, VERIFY_ROUTE,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;
use tower::limit::RateLimitLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
//use headers::HeaderValue;
use crate::{
    model::{CallToOrdersPerPartyDelegates, DelegateByCallToOrders, SpeakerByHours},
    routes::{
        call_to_orders_per_party_delegates, delegates, delegates_by_call_to_orders,
        delegates_by_call_to_orders_and_legis_period, latest_legis_inits, latest_vote_results,
        legis_inits, parties, proposals, save_email, speakers_by_hours,
        speakers_by_hours_and_legis_period, user,
    },
    DATASERVICE_URL, LEGIS_INITS_PER_PAGE, REDIS_DB,
};
use tower_http::cors::{Any, CorsLayer};

use crate::routes::*;
use crate::routes::{login, signup, verify};
use somes_common_lib::errors::*;
use somes_common_lib::*;

#[derive(Clone)]
pub struct AppState {
    // this holds some api specific state
    //verification_map: VerificationMap,
    pub redis_client: redis::Client,
    // pub somes_db_pool: deadpool_diesel::postgres::Pool,
    pub dataservice_db_pool: deadpool_diesel::postgres::Pool,
    pub dataservice_sqlx_pool: PgPool,
}

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

impl AppState {
    pub async fn new(
        redis_client: redis::Client,
        // somes_db_pool: deadpool_diesel::postgres::Pool,
        dataservice_db_pool: deadpool_diesel::postgres::Pool,
        dataservice_sqlx_pool: PgPool,
    ) -> AppState {
        AppState {
            redis_client,
            // somes_db_pool,
            dataservice_db_pool,
            dataservice_sqlx_pool,
        }
    }
}

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
    if client.get_connection().is_err() {
        error!("Could not establish redis database connection!");
        return;
    }

    info!("Established redis database connection to {REDIS_DB}.");

    #[derive(OpenApi)]
    #[openapi(
        paths(
            signup,
            verify,
            login,
            delegates,
            proposals,
            legis_inits,
            latest_legis_inits,
            latest_vote_results,
            speakers_by_hours,
            delegates_by_call_to_orders,
            speakers_by_hours_and_legis_period,
            call_to_orders_per_party_delegates,
            delegates_by_call_to_orders_and_legis_period,
            user,
            parties,
            delegate
        ),
        components(
            schemas(
                SignUpInfo, SignUpErrorResponse,
                SignUpErrorWrapper, SignUpError,
                JWTInfo, VerifyErrorResponse,
                crate::AuthError, dataservice::db::models::DbDelegate,
                DelegatesErrorResponse, dataservice::db::models::DbProposalQuery,
                DbLegislativeInitiativeQuery, LegisInitErrorResponse,
                DateRange, VoteResult,
                LegisPeriod, SpeakerByHours,
                DelegateByCallToOrders, CallToOrdersPerPartyDelegates,
                UserInfo, UserErrorResponse,
                DbParty, PartiesErrorResponse,
                DelegateById, InterestShare,
                Page
            ),
        ),
        // modifiers(&SecurityAddon),
        /*tags(
            (name = "test", description = "Todo items management API")
        )*/
    )]
    struct ApiDoc;

    /*successfully.
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(DATABASE_URL);
    let postgres_pool = bb8::Pool::builder().build(config).await.unwrap();
     */

    let dataservice_sqlx_pool = PgPoolOptions::new()
        // pool sizes
        .max_connections(100)
        .connect(DATASERVICE_URL)
        .await
        .unwrap();

    let somes_db_manager =
        // mind the database url, it is "DATASERVICE_URL" and not "DATABASE_URL"
        deadpool_diesel::postgres::Manager::new(/*DATABASE_URL*/DATASERVICE_URL, deadpool_diesel::Runtime::Tokio1);
    let somes_db_pool = deadpool_diesel::postgres::Pool::builder(somes_db_manager)
        .max_size(100)
        .build()
        .unwrap();

    let dataservice_db_manager =
        deadpool_diesel::postgres::Manager::new(DATASERVICE_URL, deadpool_diesel::Runtime::Tokio1);

    let dataservice_db_pool = deadpool_diesel::postgres::Pool::builder(dataservice_db_manager)
        .build()
        .unwrap();

    let state = AppState::new(
        client,
        // somes_db_pool,
        dataservice_db_pool,
        dataservice_sqlx_pool,
    )
    .await;

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
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
        .route(
            DELEGATES_BY_CALL_TO_ORDERS,
            get(delegates_by_call_to_orders),
        )
        .route(
            DELEGATES_BY_CALL_TO_ORDERS_AND_LEGIS_PERIOD,
            post(delegates_by_call_to_orders_and_legis_period),
        )
        .route(
            SPEAKERS_BY_HOURS_AND_LEGIS_PERIOD,
            post(speakers_by_hours_and_legis_period),
        )
        .route(PARTIES, get(parties))
        .route(
            CALL_TO_ORDERS_PER_PARTY_DELEGATES,
            get(call_to_orders_per_party_delegates),
        )
        .route(USER, post(user))
        .route(DELEGATE, get(delegate))
        .route(DELEGATE_INTERESTS, get(delegate_interests))
        .route(VOTE_RESULTS_PER_PAGE, get(vote_results_per_page))
        .route("/save_email", post(save_email))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([http::Method::GET, http::Method::POST])
                .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION]),
        )
        // .layer(RateLimitLayer::new(num, per))
        .with_state(state);
    //.with_state(verification_map)
    //.with_state(verification_hasher);
    // let config = RustlsConfig::from_pem_file(
    //     PathBuf::from("/etc/letsencrypt/live/somes.at/fullchain.pem"),
    //     PathBuf::from("/etc/letsencrypt/live/somes.at/privkey.pem"),
    // )
    // .await
    // .unwrap();

    info!("Binding API on {addr}");

    // let server = axum_server::bind_rustls(addr, config);

    let listener = match TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(e) => panic!("Could not initialize API: {e}"),
    };

    info!("Now listening..");
    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        error!("API returned error state: {e}")
    }
}
