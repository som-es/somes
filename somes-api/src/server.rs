use std::net::SocketAddr;

use axum::{
    http,
    routing::{get, post},
    Router,
};
use somes_common_lib::SIGNUP_ROUTE;
//use headers::HeaderValue;
use tower_http::cors::{Any, CorsLayer};

use crate::routes::signup;

pub async fn serve(addr: SocketAddr) {
    let app = Router::new()
        .route(SIGNUP_ROUTE, post(signup))
        //.route("/auth", post(authorize))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([http::Method::GET, http::Method::POST])
                .allow_headers([http::header::CONTENT_TYPE]),
        );

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
