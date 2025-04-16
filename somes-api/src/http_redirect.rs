use std::net::SocketAddr;

use axum::{handler::HandlerWithoutStateExt, http::Uri, response::Redirect, BoxError};
use axum_extra::extract::Host;
use reqwest::StatusCode;

#[derive(Clone, Copy)]
pub struct Ports {
    pub http: u16,
    pub https: u16,
}

pub async fn redirect_http_to_https(ports: Ports, mut sock_addr: SocketAddr) {
    sock_addr.set_port(ports.http);
    fn make_https(host: String, uri: Uri, ports: Ports) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&ports.http.to_string(), &ports.https.to_string());
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, ports) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(_error) => Err(StatusCode::BAD_REQUEST),
        }
    };

    let listener = tokio::net::TcpListener::bind(sock_addr).await.unwrap();
    log::info!("redirector listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, redirect.into_make_service())
        .await
        .unwrap();
}
