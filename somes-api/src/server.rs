use std::{error::Error, fs::File, net::SocketAddr, path::PathBuf, time::Duration};

use crate::{
    meilisearch::update_meilisearch_indices, redirect_http_to_https, reset_cache,
    routes::save_email_route, update_caches, Ports, DATASERVICE_URL, HTTPS_PORT, HTTP_PORT,
    MEILISEARCH_SECRET, MEILISEARCH_URL, PRIVATE_KEY_PATH, PUBLIC_KEY_PATH, REDIS_DB,
    STATIC_FRONTEND_PATH,
};
use crate::{routes::*, IS_PROD};
use axum::{
    extract::FromRef,
    http::{self, HeaderValue},
    response::Html,
    routing::{any, get, get_service, post},
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use log::info;
use reqwest::StatusCode;
use somes_common_lib::*;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::{net::TcpListener, time::sleep};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, cors::AllowOrigin, cors::CorsLayer,
    decompression::RequestDecompressionLayer, services::ServeDir,
};
use views::{create_composite_types, create_views};

type ServerResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

const DEFAULT_ALLOWED_ORIGINS: [&str; 2] = ["https://somes.at", "https://www.somes.at"];
const ASSET_REFRESH_DELAY: Duration = Duration::from_secs(19_000);

#[derive(Clone)]
pub struct AppState {
    pub redis_client: redis::Client,
    pub dataservice_sqlx_pool: PgPool,
    pub meilisearch_client: meilisearch_sdk::client::Client,
}

impl AppState {
    pub fn new(
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

fn allowed_cors_origin() -> AllowOrigin {
    if !*IS_PROD {
        return AllowOrigin::any();
    }

    match std::env::var("CORS_ALLOWED_ORIGINS") {
        Ok(origins) => {
            let origins = origins
                .split(',')
                .filter_map(|origin| origin.trim().parse::<HeaderValue>().ok())
                .collect::<Vec<_>>();

            (!origins.is_empty())
                .then(|| AllowOrigin::list(origins))
                .unwrap_or_else(default_allowed_origins)
        }
        Err(_) => default_allowed_origins(),
    }
}

fn default_allowed_origins() -> AllowOrigin {
    AllowOrigin::list(DEFAULT_ALLOWED_ORIGINS.map(HeaderValue::from_static))
}

fn connect_redis() -> ServerResult<redis::Client> {
    let client = redis::Client::open(REDIS_DB)?;
    client.get_connection()?;
    if reset_cache() || *IS_PROD {
        let mut con = client.get_connection()?;
        redis::cmd("FLUSHALL").query::<()>(&mut con)?;
    }
    info!("Established redis database connection to {REDIS_DB}.");
    Ok(client)
}

async fn connect_dataservice() -> ServerResult<PgPool> {
    log::info!(
        "Connecting to database {}",
        DATASERVICE_URL.split("@").last().unwrap_or_default()
    );

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(DATASERVICE_URL)
        .await?;

    log::info!("Established postgresql connection");
    Ok(pool)
}

async fn maybe_update_views(dataservice_sqlx_pool: &PgPool) -> ServerResult<()> {
    if std::env::var("UPDATE_VIEWS").unwrap_or_else(|_| "false".to_string()) != "true" {
        return Ok(());
    }

    let mut tx = dataservice_sqlx_pool.begin().await?;
    if let Err(e) = create_composite_types(&mut tx).await {
        log::error!("Cannot create composite types: {e:?}");
    }
    if let Err(e) = create_views(&mut tx).await {
        log::error!("Cannot create views: {e:?}");
    }
    tx.commit().await?;
    Ok(())
}

fn spawn_asset_refresh(pg_pool: PgPool) {
    tokio::task::spawn(async move {
        if tokio::fs::try_exists("assets").await.unwrap_or_default() {
            sleep(ASSET_REFRESH_DELAY).await;
        }
        if let Err(e) = update_delegate_assets(&pg_pool).await {
            log::error!("Could not download assets {e:?}");
        }
        sleep(ASSET_REFRESH_DELAY).await;
    });
}

fn spawn_search_refresh(
    client: redis::Client,
    dataservice_sqlx_pool: PgPool,
    meilisearch_client: meilisearch_sdk::client::Client,
) {
    tokio::task::spawn(async move {
        // This function blocks in production so streamed updates do not invalidate in-flight fetches
        // and leave Meilisearch with outdated indices.
        update_meilisearch_indices(&client, &dataservice_sqlx_pool, &meilisearch_client).await;

        if *IS_PROD {
            update_caches(&client, &dataservice_sqlx_pool, &meilisearch_client);
        }
    });
}

fn api_router() -> Router<AppState> {
    let at_routes = Router::new()
        .route(PARTIES, get(parties_route))
        .route(PARTIES_AT_GP, get(parties_at_gp_route))
        .route(PARTIES_PER_GP, get(parties_per_gp_route))
        .route(
            COALITION_PARTIES_PER_GP,
            get(coalition_parties_per_gp_route),
        )
        .route(DEPARTMENTS, get(departments))
        .route(DEPARTMENTS_PER_GP, get(departments_per_gp))
        .route(ALL_GPS, get(all_gps_route))
        .route(SEATS, get(seats_route))
        .route(TOPICS, get(topics_route))
        .route(EUROVOC_TOPICS, get(eurovoc_topics_route))
        .route(AI_CHAT_WS, any(ai_chat_ws_handler_route))
        .route(NEXT_PLENAR_DATE, get(next_plenar_date_route))
        .route(PLENAR_DATES, get(plenar_dates_route))
        .route(PLENARY_SESSIONS_PER_GP, get(plenary_sessions_per_gp_route))
        .route("/v1/questions", get(all_delegate_questions_route))
        .route(
            "/v1/admin/delegate-questions/pending",
            get(pending_delegate_questions_route),
        )
        .route(
            "/v1/admin/delegate-questions/{question_id}/approve",
            post(approve_delegate_question_route),
        )
        .route(
            "/v1/admin/delegate-questions/{question_id}/reject",
            post(reject_delegate_question_route),
        )
        .route("/save_email", post(save_email_route))
        .nest("/v1/statistics", create_statistics_router())
        .nest("/v1/delegates", create_delegates_router())
        .nest("/v1/gov_proposals", create_gov_proposals_router())
        .nest("/v1/decrees", create_decrees_router())
        .nest("/v1/user", create_user_router())
        .nest("/v1/vote_results", create_vote_results_router())
        .nest("/v1/events", create_events_router());

    Router::new()
        .route("/oauth/{provider}", get(start_oauth))
        .route("/oauth/{provider}/callback", get(oauth_callback))
        .route(WALO_QUESTIONS, get(walo_questions_route))
        .route(QUIZZES, get(get_all_quizzes_route))
        .route(ADD_QUIZ, post(add_quiz_route))
        .route(QUIZ_ROOM, any(join_quiz_room_route))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest("/at", at_routes)
}

fn app_router(state: AppState) -> Router {
    let static_files_dir = PathBuf::from(STATIC_FRONTEND_PATH);
    let current_frontend_dir = ServeDir::new(static_files_dir)
        .fallback(get(|| async { Html(include_str!("../build/index.html")) }));

    let landing_server_dir = ServeDir::new("somes-landing").fallback(get(|| async {
        Html(include_str!("../somes-landing/index.html"))
    }));

    Router::new()
        .nest("/api", api_router())
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
        .layer(
            CorsLayer::new()
                .allow_origin(allowed_cors_origin())
                .allow_methods([
                    http::Method::GET,
                    http::Method::POST,
                    http::Method::DELETE,
                    http::Method::PUT,
                ])
                .allow_headers([
                    http::header::CONTENT_TYPE,
                    http::header::AUTHORIZATION,
                    http::header::ACCEPT_ENCODING,
                ]),
        )
        .layer(
            ServiceBuilder::new()
                .layer(RequestDecompressionLayer::new())
                .layer(CompressionLayer::new()),
        )
        .with_state(state)
}

async fn serve_plain(addr: SocketAddr, app: Router) -> ServerResult<()> {
    info!("Binding API on {addr}");
    let listener = TcpListener::bind(&addr).await?;

    info!("Now listening..");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}

async fn serve_tls(addr: SocketAddr, config: RustlsConfig, app: Router) -> ServerResult<()> {
    let ports = Ports {
        http: HTTP_PORT.parse()?,
        https: HTTPS_PORT.parse()?,
    };
    let mut sock_addr = addr;
    tokio::spawn(redirect_http_to_https(ports, sock_addr));

    sock_addr.set_port(ports.https);

    info!("Binding API on {sock_addr}");
    axum_server::bind_rustls(sock_addr, config)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;
    Ok(())
}

pub async fn serve(addr: SocketAddr) -> ServerResult<()> {
    let client = connect_redis()?;
    let dataservice_sqlx_pool = connect_dataservice().await?;

    let meilisearch_client =
        meilisearch_sdk::client::Client::new(MEILISEARCH_URL, Some(MEILISEARCH_SECRET))?;

    let state = AppState::new(
        client.clone(),
        dataservice_sqlx_pool.clone(),
        meilisearch_client.clone(),
    );

    spawn_asset_refresh(dataservice_sqlx_pool.clone());
    maybe_update_views(&dataservice_sqlx_pool).await?;

    crate::refresh_views(&dataservice_sqlx_pool, &client);

    spawn_search_refresh(client, dataservice_sqlx_pool, meilisearch_client);

    let config = RustlsConfig::from_pem_file(
        PathBuf::from(PUBLIC_KEY_PATH),
        PathBuf::from(PRIVATE_KEY_PATH),
    )
    .await;

    let app = app_router(state);

    if std::env::var("SOMES_DEBUG").unwrap_or_default() == "DEBUG" {
        return serve_plain(addr, app).await;
    }

    match config {
        Ok(config) => serve_tls(addr, config, app).await,
        Err(_) => serve_plain(addr, app).await,
    }
}

async fn update_delegate_assets(pg_pool: &sqlx::Pool<sqlx::Postgres>) -> ServerResult<()> {
    let _ = tokio::fs::create_dir("assets").await;

    let img_urls = sqlx::query!("select id, image_url from delegates where image_url is not null")
        .fetch_all(pg_pool)
        .await?;

    tokio::task::spawn_blocking(move || {
        let client = reqwest::blocking::Client::new();
        for img_url in img_urls {
            let Some(image_url) = img_url.image_url else {
                continue;
            };
            let Ok(mut res) = client.get(&image_url).send() else {
                log::warn!("Could not download delegate asset from {image_url}");
                continue;
            };

            let path = format!("assets/{}.jpg", img_url.id);
            let Ok(mut file) = File::create(&path) else {
                log::warn!("Could not create delegate asset file {path}");
                continue;
            };
            if let Err(e) = res.copy_to(&mut file) {
                log::warn!("Could not save delegate asset {path}: {e:?}");
            }
        }
    })
    .await?;

    Ok(())
}
