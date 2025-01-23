use std::{error::Error, fs::File, net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::FromRef,
    http::{self, HeaderValue},
    response::Html,
    routing::{any, delete, get, get_service, post, Route},
    Router, ServiceExt,
};
use axum_server::tls_rustls::RustlsConfig;
use dataservice::db::models::{DbLegislativeInitiativeQuery, DbParty};
use diesel::dsl::host;
// use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use log::{error, info};
use meilisearch_sdk::settings::Settings;
use redis::aio::MultiplexedConnection;
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
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
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
        client.clone(),
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
            if let Err(e) = update_meilisearch_index(
                &mut client.get_multiplexed_tokio_connection().await.unwrap(),
                &pg_pool,
                &meilisearch_client,
            )
            .await
            {
                log::warn!("Could not update meilisearch index: {e:?}");
            }
            sleep(std::time::Duration::from_secs(1900)).await;
        }
    });
    let pg_pool = dataservice_sqlx_pool.clone();

    tokio::task::spawn(async move {
        if tokio::fs::try_exists("assets").await.unwrap_or_default() {
            sleep(std::time::Duration::from_secs(19000)).await
        }
        if let Err(e) = update_delegate_assets(&pg_pool).await {
            log::error!("Could not download assets {e:?}");
        }
        sleep(std::time::Duration::from_secs(19000)).await;
        // loop {

        // }
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

    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(4)
            .finish()
            .unwrap(),
    );

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route(SIGNUP_ROUTE, post(signup))
        .route(VERIFY_ROUTE, get(verify)) // or post?
        .route(
            LOGIN_ROUTE,
            post(login).layer(GovernorLayer {
                config: governor_conf.clone(),
            }),
        )
        .route(DELETE_ACCOUNT_ROUTE, delete(delete_account))
        .route(DELEGATES_ROUTE, get(delegates))
        .route(PROPOSALS_ROUTE, get(proposals))
        // .route(LEGIS_INIT_ROUTE, post(legis_inits))
        // .route(LATEST_LEGIS_INITS_ROUTE, get(latest_legis_inits))
        .route(LATEST_VOTE_RESULTS_ROUTE, get(latest_vote_results))
        // statistics
        .route(SPEAKERS_BY_HOURS, get(speakers_by_hours))
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
        .route(
            SPEECHES_BY_DELEGATE_PER_PAGE,
            get(speeches_by_delegate_per_page),
        ) // post only because js fetch...
        .route(VOTE_RESULT_BY_ID, get(vote_result_by_id)) // post only because js fetch...
        .route(VOTE_RESULT_BY_SEARCH, post(vote_result_by_search)) // post only because js fetch...
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
        .route(GOV_OFFICIALS_AT, get(gov_officials_at_date_route))
        .route(GOV_PROPOSALS_BY_OFFICIAL, get(gov_proposals_by_official))
        .route(DELEGATE_POLITICAL_POSITION, get(delegate_political_position))
        .route(AI_CHAT_WS, any(ai_chat_ws_handler))
        .route(
            DELEGATES_BY_CALL_TO_ORDERS,
            post(call_to_order_per_delegates),
        )
        .route(
            LEGISLATIVE_INITIATIVES_WITHOUT_SIMPLE_MAJORITY,
            post(legislative_initiatives_without_simple_majority),
        )
        .route(COMPLEXITY_PER_DELEGATE, post(complexity_per_delegate))
        .route(COMPLEXITY_PER_PARTY, post(complexity_per_party))
        .route(COMPLEXITY_PER_GENDER, post(complexity_per_gender))
        .route(COMPLEXITY_AT_AGE, post(complexity_at_age))
        .route(AGE_OF_DELEGATES, post(age_per_delegate))
        .route(AGE_PER_PARTY, post(age_per_party))
        .route(SPEECHTIME_PER_PARTY, post(speechtime_per_party))
        .route(SPEECHTIME_PER_DELEGATE, post(speechtime_per_delegate))
        .route(SPEECHTIME_PER_AGE, post(speechtime_per_age))
        .route(SPEECHTIME_PER_GENDER, post(speechtime_per_gender))
        .route(
            TOTAL_SPEECHES_PER_DELEGATE,
            post(total_speeches_per_delegate),
        )
        .route(TOTAL_SPEECHES_PER_PARTY, post(total_speeches_per_party))
        .route(TOTAL_SPEECHES_PER_GENDER, post(total_speeches_per_gender))
        .route(TOTAL_SPEECHES_PER_AGE, post(total_speeches_per_age))
        .route(
            CALL_TO_ORDERS_BY_DELEGATE,
            post(call_to_orders_per_delegate),
        )
        .route(CALL_TO_ORDERS_PER_PARTY, post(call_to_orders_per_party))
        .route(CALL_TO_ORDERS_PER_GENDER, post(call_to_orders_per_gender))
        .route(CALL_TO_ORDERS_PER_AGE, post(call_to_orders_per_age))
        .route(DIVISION_ACCURACY_SCORE_PER_DELEGATE, post(divison_accuracy_score_per_delegate))
        .route(DIVISION_ACCURACY_SCORE_PER_PARTY, post(division_accuracy_score_per_party))
        .route(DIVISION_ACCURACY_SCORE_PER_GENDER, post(division_accuracy_score_per_gender))
        .route(DIVISION_ACCURACY_SCORE_PER_AGE, post(division_accuracy_score_per_age))
        .route(VOTES_TOGETHER, post(votes_together))
        .route(ABSENCES_PER_DELEGATE, post(absences_per_delegate))
        .route(ABSENCES_PER_PARTY, post(absences_per_party))
        .route(ABSENCES_PER_GENDER, post(absences_per_gender))
        .route(ABSENCES_PER_AGE, post(absences_per_age))
        .route("/save_email", post(save_email))
        .nest_service("/assets", ServeDir::new("assets"))
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
                .serve(app.into_make_service_with_connect_info::<SocketAddr>())
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
            if let Err(e) = axum::serve(
                listener,
                app.into_make_service_with_connect_info::<SocketAddr>(),
            )
            .await
            {
                error!("API returned error state: {e}")
            }
        }
    }
}

async fn update_delegate_assets(
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<(), Box<dyn Error>> {
    let _ = tokio::fs::create_dir("assets").await;

    let img_urls = sqlx::query!("select id, image_url from delegates where image_url is not null")
        .fetch_all(pg_pool)
        .await?;

    std::thread::spawn(move || {
        let client = reqwest::blocking::Client::new();
        for img_url in img_urls {
            let mut file = File::create(format!("assets/{}.jpg", img_url.id)).unwrap();
            client
                .get(&img_url.image_url.unwrap())
                .send()
                .unwrap()
                .copy_to(&mut file)
                .unwrap();
        }
    });
    Ok(())
}

async fn update_meilisearch_index(
    redis_con: &mut MultiplexedConnection,
    pg_pool: &sqlx::Pool<sqlx::Postgres>,
    client: &meilisearch_sdk::client::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Fetching all vote results..");
    let all_vote_results = get_all_votes_from_legis_init(redis_con.clone(), pg_pool).await?;
    log::info!("Fetched all vote results");

    // client.delete_index("vote_results").await?;

    log::info!(
        "Uploading {} vote results to meilisearch",
        all_vote_results.len()
    );
    let settings = Settings::new().with_filterable_attributes([
        "legislative_initiative.accepted",
        "legislative_initiative.requires_simple_majority",
        "legislative_initiative.gp",
        "legislative_initiative.voted_by_name",
    ]);

    client.index("vote_results").set_settings(&settings).await?;

    client
        .index("vote_results")
        .add_documents(&all_vote_results, Some("id"))
        .await?;

    log::info!("Uploaded vote results");
    Ok(())
}
