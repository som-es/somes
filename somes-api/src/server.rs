use std::{net::SocketAddr, path::PathBuf};

use axum::{
    extract::FromRef,
    http::{self, HeaderValue},
    response::Html,
    routing::{delete, get, get_service, post},
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use dataservice::db::models::{DbLegislativeInitiativeQuery, DbParty};
// use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use log::{error, info};
use reqwest::StatusCode;
use somes_common_lib::{
    CALL_TO_ORDERS_PER_PARTY_DELEGATES, DELEGATES_BY_CALL_TO_ORDERS,
    DELEGATES_BY_CALL_TO_ORDERS_AND_LEGIS_PERIOD, DELEGATES_ROUTE, LATEST_LEGIS_INITS_ROUTE,
    LATEST_VOTE_RESULTS_ROUTE, LEGIS_INIT_ROUTE, LOGIN_ROUTE, PARTIES, PROPOSALS_ROUTE,
    SIGNUP_ROUTE, SPEAKERS_BY_HOURS, SPEAKERS_BY_HOURS_AND_LEGIS_PERIOD, USER, VERIFY_ROUTE,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::{net::TcpListener, time::sleep};
use tower::limit::RateLimitLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
//use headers::HeaderValue;
use crate::{
    model::{CallToOrdersPerPartyDelegates, DelegateByCallToOrders, SpeakerByHours},
    redirect_http_to_https,
    routes::{
        call_to_orders_per_party_delegates, delegates, delegates_by_call_to_orders,
        delegates_by_call_to_orders_and_legis_period, latest_vote_results, parties, proposals,
        save_email, speakers_by_hours, speakers_by_hours_and_legis_period, user,
    },
    Ports, DATASERVICE_URL, HTTPS_PORT, HTTP_PORT, LEGIS_INITS_PER_PAGE, MEILISEARCH_SECRET,
    MEILISEARCH_URL, PRIVATE_KEY_PATH, PUBLIC_KEY_PATH, REDIS_DB, STATIC_FRONTEND_PATH,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

use crate::jwt::*;
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
    pub meilisearch_client: meilisearch_sdk::client::Client,
}

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

impl AppState {
    pub async fn new(
        redis_client: redis::Client,
        // somes_db_pool: deadpool_diesel::postgres::Pool,
        dataservice_db_pool: deadpool_diesel::postgres::Pool,
        dataservice_sqlx_pool: PgPool,
        meilisearch_client: meilisearch_sdk::client::Client,
    ) -> AppState {
        AppState {
            redis_client,
            // somes_db_pool,
            dataservice_db_pool,
            dataservice_sqlx_pool,
            meilisearch_client,
        }
    }
}

impl FromRef<AppState> for redis::Client {
    fn from_ref(app_state: &AppState) -> redis::Client {
        app_state.redis_client.clone()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Test {
    test: &'static str,
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
        .max_connections(20)
        .connect(DATASERVICE_URL)
        .await
        .unwrap();

    let somes_db_manager =
        // mind the database url, it is "DATASERVICE_URL" and not "DATABASE_URL"
        deadpool_diesel::postgres::Manager::new(/*DATABASE_URL*/DATASERVICE_URL, deadpool_diesel::Runtime::Tokio1);
    // let somes_db_pool = deadpool_diesel::postgres::Pool::builder(somes_db_manager)
    //     .max_size(100)
    //     .build()
    //     .unwrap();

    let dataservice_db_manager =
        deadpool_diesel::postgres::Manager::new(DATASERVICE_URL, deadpool_diesel::Runtime::Tokio1);

    let dataservice_db_pool = deadpool_diesel::postgres::Pool::builder(dataservice_db_manager)
        .build()
        .unwrap();

    let meilisearch_client =
        meilisearch_sdk::client::Client::new(MEILISEARCH_URL, Some(MEILISEARCH_SECRET))
            .expect("Meilisearch client was not able to connect");

    meilisearch_client
        .index("test")
        .add_documents(&[Test { test: "test" }], None)
        .await
        .unwrap();

    let state = AppState::new(
        client,
        // somes_db_pool,
        dataservice_db_pool,
        dataservice_sqlx_pool.clone(),
        meilisearch_client.clone(),
    )
    .await;

    let static_files_dir = PathBuf::from(STATIC_FRONTEND_PATH);
    let current_frontend_dir = ServeDir::new(static_files_dir.clone())
        .fallback(get(|| async { Html(include_str!("../build/index.html")) }));

    let static_files_dir_alpha_0_1 = PathBuf::from("./build-alpha-0.1");
    let alpha_0_1_frontend_dir =
        ServeDir::new(static_files_dir_alpha_0_1.clone()).fallback(get(|| async {
            Html(include_str!("../build-alpha-0.1/index.html"))
        }));

    let pg_pool = dataservice_sqlx_pool.clone();

    // TODO: move this to dataservice
    tokio::task::spawn(async move {
        loop {
            if let Err(e) = update_meilisearch_index(&pg_pool, &meilisearch_client).await {
                log::warn!("Could not update meilisearch index: {e:?}");
            }
            sleep(std::time::Duration::from_secs(1900)).await;
        }
    });

    let config = RustlsConfig::from_pem_file(
        PathBuf::from(PUBLIC_KEY_PATH),
        PathBuf::from(PRIVATE_KEY_PATH),
    )
    .await;

    let landing_server_dir = ServeDir::new("../deploy-rs/somes-landing");
    let landing_app = Router::new().nest_service("/", landing_server_dir);
    // let origins = [
    //     "https://somes.at".parse::<HeaderValue>().unwrap(),
    //     "https://somes.at".parse::<HeaderValue>().unwrap(),
    // ];

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route(SIGNUP_ROUTE, post(signup))
        .route(VERIFY_ROUTE, get(verify)) // or post?
        .route(LOGIN_ROUTE, post(login))
        .route(DELEGATES_ROUTE, get(delegates))
        .route(PROPOSALS_ROUTE, get(proposals))
        // .route(LEGIS_INIT_ROUTE, post(legis_inits))
        // .route(LATEST_LEGIS_INITS_ROUTE, get(latest_legis_inits))
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
        .route(DELEGATE_QA, get(delegate_qa))
        .route(VOTE_RESULTS_PER_PAGE, post(vote_results_per_page)) // post only because js fetch...
        .route(VOTE_RESULT_BY_ID, get(vote_result_by_id)) // post only because js fetch...
        .route(VOTE_RESULT_BY_SEARCH, get(vote_result_by_search)) // post only because js fetch...
        .route(DELEGATES_AT, get(delegates_at)) // post only because js fetch...
        .route(
            DELEGATES_WITH_SEATS_NEAR_DATE,
            get(delegates_with_seats_near_date_route),
        )
        .route(WALO_QUESTIONS, get(walo_questions))
        .route(ALL_GPS, get(all_gps))
        .route(SEATS, get(seats))
        .route(RENEW_TOKEN, post(renew_token))
        .route(TOPICS, get(topics))
        .route(TOPIC_SELECTION, post(add_user_topic))
        .route(TOPIC_SELECTION, delete(remove_user_topic))
        .route(TOPIC_SELECTION, get(user_topic_selection))
        .route(AI_CHAT_WS, get(ai_chat_ws_handler))
        .route("/save_email", post(save_email))
        // mind conflicts e.g delegates
        .nest_service(
            "/alpha",
            get_service(current_frontend_dir).handle_error(|_| async move {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }),
        )
        .nest_service(
            "/alpha-0.1",
            get_service(alpha_0_1_frontend_dir).handle_error(|_| async move {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }),
        )
        .nest("/", landing_app)
        // .fallback_service(get_service(serve_dir).handle_error(|_| async move {
        //     (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        // }))
        .layer(
            CorsLayer::new()
                // .allow_origin("https://somes.at".parse::<HeaderValue>().unwrap())
                .allow_origin(Any)
                .allow_methods([http::Method::GET, http::Method::POST, http::Method::DELETE])
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

    // let server = axum_server::bind_rustls(addr, config);
    match config {
        Ok(config) => {
            let ports = Ports {
                http: HTTP_PORT.parse().unwrap(),
                https: HTTPS_PORT.parse().unwrap(),
            };
            let mut sock_addr = addr;
            tokio::spawn(redirect_http_to_https(ports, sock_addr));

            sock_addr.set_port(ports.https);

            info!("Binding API on {sock_addr}");
            axum_server::bind_rustls(sock_addr, config.clone())
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
        Err(_) => {
            info!("Binding API on {addr}");
            let listener = match TcpListener::bind(&addr).await {
                Ok(listener) => listener,
                Err(e) => panic!("Could not initialize API: {e}"),
            };

            info!("Now listening..");
            if let Err(e) = axum::serve(listener, app.into_make_service()).await {
                error!("API returned error state: {e}")
            }
        }
    }
}

async fn update_meilisearch_index(
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    client: &meilisearch_sdk::client::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Fetching all vote results..");
    let all_vote_results = get_all_votes_from_legis_init(pg_pool).await?;
    log::info!("Fetched all vote results");

    // client.delete_index("vote_results").await?;

    log::info!(
        "Uploading {} vote results to meilisearch",
        all_vote_results.len()
    );

    client
        .index("vote_results")
        .add_documents(&all_vote_results, Some("id"))
        .await?;

    log::info!("Uploaded vote results");
    Ok(())
}
