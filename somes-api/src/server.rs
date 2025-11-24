use std::{error::Error, fs::File, net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::FromRef,
    http::{self},
    response::Html,
    routing::{any, delete, get, get_service, post, put},
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use dataservice::{
    combx::VoteResult,
    db::models::{DbLegislativeInitiativeQuery, DbParty},
};
// use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use log::{error, info};
use reqwest::StatusCode;
use somes_common_lib::{
    DECREES_PER_PAGE, DELEGATES_BY_CALL_TO_ORDERS, DELEGATES_ROUTE, LATEST_VOTE_RESULTS_ROUTE,
    LOGIN_ROUTE, PROPOSALS_ROUTE,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::{net::TcpListener, time::sleep};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use utoipa::OpenApi;
use views::{create_composite_types, create_views};
//use headers::HeaderValue;
use crate::{
    meilisearch::update_meilisearch_indices,
    redirect_http_to_https,
    routes::{delegates, latest_vote_results, proposals, save_email},
    Ports, DATASERVICE_URL, HTTPS_PORT, HTTP_PORT, MEILISEARCH_SECRET, MEILISEARCH_URL,
    PRIVATE_KEY_PATH, PUBLIC_KEY_PATH, REDIS_DB, STATIC_FRONTEND_PATH,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

use crate::jwt::*;
use crate::routes::login;
use crate::routes::*;
use somes_common_lib::errors::*;
use somes_common_lib::*;

#[derive(Clone)]
pub struct AppState {
    pub redis_client: redis::Client,
    pub dataservice_sqlx_pool: PgPool,
    pub meilisearch_client: meilisearch_sdk::client::Client,
}

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

impl AppState {
    pub async fn new(
        redis_client: redis::Client,
        dataservice_sqlx_pool: PgPool,
        meilisearch_client: meilisearch_sdk::client::Client,
    ) -> AppState {
        AppState {
            redis_client,
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

    #[cfg(debug_assertions)]
    {
        let mut con = client.get_connection().unwrap();
        redis::cmd("FLUSHALL").query::<()>(&mut con).unwrap();
    }

    info!("Established redis database connection to {REDIS_DB}.");

    #[derive(OpenApi)]
    #[openapi(
        paths(
            login,
            delegates,
            proposals,
            latest_vote_results,
            parties,
            delegate_by_id
        ),
        components(
            schemas(
                SignUpInfo,
                SignUpError,
                JWTInfo,
                crate::AuthError, dataservice::db::models::DbDelegate,
                DelegatesErrorResponse, dataservice::db::models::DbProposalQuery,
                DbLegislativeInitiativeQuery, LegisInitErrorResponse,
                DateRange, VoteResult,
                LegisPeriod,
                UserInfo,
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

    log::info!(
        "Connecting to database {}",
        DATASERVICE_URL.split("@").last().unwrap_or_default()
    );

    let dataservice_sqlx_pool = PgPoolOptions::new()
        // pool sizes
        .max_connections(20)
        .connect(DATASERVICE_URL)
        .await
        .unwrap();

    log::info!("Established postgresql connection");

    // let somes_db_pool = deadpool_diesel::postgres::Pool::builder(somes_db_manager)
    //     .max_size(100)
    //     .build()
    //     .unwrap();

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
        dataservice_sqlx_pool.clone(),
        meilisearch_client.clone(),
    )
    .await;

    let static_files_dir = PathBuf::from(STATIC_FRONTEND_PATH);
    let current_frontend_dir = ServeDir::new(static_files_dir.clone())
        .fallback(get(|| async { Html(include_str!("../build/index.html")) }));

    let static_files_dir_alpha_0_1 = PathBuf::from("./build-alpha-0.1");
    let pg_pool = dataservice_sqlx_pool.clone();

    tokio::task::spawn(async move {
        if tokio::fs::try_exists("assets").await.unwrap_or_default() {
            sleep(std::time::Duration::from_secs(19000)).await
        }
        if let Err(e) = update_delegate_assets(&pg_pool).await {
            log::error!("Could not download assets {e:?}");
        }
        sleep(std::time::Duration::from_secs(19000)).await;
    });

    let update_views = std::env::var("UPDATE_VIEWS").unwrap_or_else(|_| "false".to_string());
    if update_views == "true" {
        let mut tx = dataservice_sqlx_pool.begin().await.unwrap();
        if let Err(e) = create_composite_types(&mut tx).await {
            log::error!("Cannot create composite types: {e:?}")
        }
        if let Err(e) = create_views(&mut tx).await {
            log::error!("Cannot create views: {e:?}")
        }
        tx.commit().await.unwrap();
    }

    update_meilisearch_indices(client, dataservice_sqlx_pool, meilisearch_client);

    let config = RustlsConfig::from_pem_file(
        PathBuf::from(PUBLIC_KEY_PATH),
        PathBuf::from(PRIVATE_KEY_PATH),
    )
    .await;

    let landing_server_dir = ServeDir::new("../deploy-rs/somes-landing").fallback(get(|| async {
        Html(include_str!("../../deploy-rs/somes-landing/index.html"))
    }));
    // let landing_app = Router::new().nest_service("/", landing_server_dir);
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

    let api_routes = Router::new()
        .route(
            LOGIN_ROUTE,
            post(login).layer(GovernorLayer {
                config: governor_conf.clone(),
            }),
        )
        .route(DELETE_ACCOUNT_ROUTE, delete(delete_account))
        .route(PROPOSALS_ROUTE, get(proposals))
        // .route(LEGIS_INIT_ROUTE, post(legis_inits))
        // .route(LATEST_LEGIS_INITS_ROUTE, get(latest_legis_inits))
        .route(LATEST_VOTE_RESULTS_ROUTE, get(latest_vote_results))
        .route(
            LATEST_MINISTRIAL_PROPOSALS,
            get(latest_ministrial_proposals),
        )
        .route(PARTIES, get(parties))
        .route(PARTIES_AT_GP, get(parties_at_gp))
        .route(USER, get(user))
        .route(DELEGATE_INTERESTS, get(delegate_interests))
        .route(VOTE_RESULTS_PER_PAGE, post(vote_results_per_page)) // post only because js fetch...
        .route(
            UNFINISHED_VOTE_RESULTS_PER_PAGE,
            post(unfinished_vote_results_per_page),
        ) // post only because js fetch...
        .route(VOTE_RESULT_BY_ID, get(vote_result_by_id))
        .route(VOTE_RESULT_BY_PATH, get(vote_result_by_path))
        .route(VOTE_RESULT_BY_SEARCH, post(vote_result_by_search)) // post only because js fetch...
        .route(
            UNFINISHED_VOTE_RESULT_BY_SEARCH,
            post(unfinished_vote_result_by_search),
        ) // post only because js fetch...
        .route(WALO_QUESTIONS, get(walo_questions))
        .route(ALL_GPS, get(all_gps))
        .route(SEATS, get(seats))
        .route(RENEW_TOKEN, post(renew_token))
        .route(TOPICS, get(topics))
        .route(EUROVOC_TOPICS, get(eurovoc_topics))
        .route(TOPIC_SELECTION, post(add_user_topic))
        .route(TOPIC_SELECTION, delete(remove_user_topic))
        .route(TOPIC_SELECTION, get(user_topic_selection))
        .route(FAVO_DELEGATE, post(add_delegate_favo))
        .route(FAVO_DELEGATE, get(user_delegate_favos))
        .route(FAVO_DELEGATE, delete(remove_user_delegate_favo))
        .route(FAVO_LEGIS_INIT, delete(remove_user_legis_init_favo))
        .route(FAVO_LEGIS_INIT, get(user_legis_init_favos))
        .route(FAVO_LEGIS_INIT, post(add_legis_init_favo))
        .route(SEND_MAIL_INFO, put(update_send_mail_info))
        .route(SEND_MAIL_INFO, get(get_send_mail_info))
        .route(GOV_PROPOSALS_BY_OFFICIAL, get(gov_proposals_by_official))
        .route(GOV_PROPOSALS_PER_PAGE, post(get_gov_proposals_per_page))
        .route(GOV_PROPOSALS_BY_SEARCH, post(gov_props_by_search)) // post only because js fetch...
        .route(GOV_PROPOSAL_BY_PATH, get(gov_proposal_by_path))
        .route(DECREES_PER_PAGE, post(get_decrees_per_page))
        .route(DECREES_BY_SEARCH, post(decrees_by_search))
        .route(DECREE_BY_RIS_ID, get(decree_by_ris_id))
        .route(AI_CHAT_WS, any(ai_chat_ws_handler))
        .route(QUIZZES, get(get_all_quizzes))
        .route(ADD_QUIZ, post(add_quiz))
        .route(QUIZ_ROOM, any(join_quiz_room))
        .route(NEXT_PLENAR_DATE, get(next_plenar_date))
        .route(PLENAR_DATES, get(plenar_dates))
        .route("/save_email", post(save_email))
        .nest("/statistics", create_statistics_router())
        .nest("/delegates", create_delegates_router());

    let app = Router::new()
        .nest("/api", api_routes)
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest_service("/assets", ServeDir::new("assets"))
        // mind conflicts e.g delegates
        .nest_service(
            "/alpha",
            get_service(current_frontend_dir).handle_error(|_| async move {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }),
        )
        .fallback_service(
            get_service(landing_server_dir).handle_error(|_| async move {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }),
        )
        // .nest("/", landing_app)
        // .fallback_service(get_service(serve_dir).handle_error(|_| async move {
        //     (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        // }))
        .layer(
            CorsLayer::new()
                // .allow_origin("https://somes.at".parse::<HeaderValue>().unwrap())
                .allow_origin(Any)
                .allow_methods([
                    http::Method::GET,
                    http::Method::POST,
                    http::Method::DELETE,
                    http::Method::PUT,
                ])
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

    if std::env::var("SOMES_DEBUG").unwrap_or_default() == "DEBUG" {
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
        return;
    }

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
            let Ok(mut res) = client.get(&img_url.image_url.unwrap()).send() else {
                continue;
            };

            let mut file = File::create(format!("assets/{}.jpg", img_url.id)).unwrap();
            res.copy_to(&mut file).unwrap();
        }
    });
    Ok(())
}
