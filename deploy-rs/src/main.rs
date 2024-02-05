use axum::extract::Host;
use axum::handler::{Handler, HandlerWithoutStateExt};
use axum::http::{StatusCode, Uri};
use axum::response::Redirect;
use axum::{BoxError, Router};
use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;
use tokio::net::TcpListener;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::str::FromStr;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
struct Opt {
    /// set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "0.0.0.0")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "443")]
    port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "../somes-svelte/build/")]
    static_dir: String,

    #[clap(
        long = "cert",
        default_value = "/etc/letsencrypt/live/somes.at/fullchain.pem"
    )]
    cert: String,

    #[clap(
        long = "key",
        default_value = "/etc/letsencrypt/live/somes.at/privkey.pem"
    )]
    key: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    // Setup logging & RUST_LOG from args
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }
    // enable console logging
    tracing_subscriber::fmt::init();

    // Change the base path as the alpha is accessed with /alpha
    // svelte.config.js
    // export default {
    //     kit: {
    //     paths: {
    //         base: '/alpha',
    //     },
    //     // ... other configuration options
    //     },
    // };
    
    // https://stackoverflow.com/questions/71527665/building-svelte-app-as-a-set-of-static-files
    let alpha_serve_dir = ServeDir::new(opt.static_dir);
    // let alpha_app = Router::new().nest_service("/alpha", alpha_serve_dir);
    let alpha_app = Router::new().nest_service("/", alpha_serve_dir);

    let landing_server_dir = ServeDir::new("./somes-landing");
    let landing_app = Router::new().nest_service("/", landing_server_dir);

    // let app = Router::new().merge(alpha_app).merge(landing_app);
    // let app = Router::new()
    //     //.route("/api/hello", get(hello))
    //     .merge(SpaRouter::new("/assets", opt.static_dir))
    //     .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    let ports = Ports {
        http: 80,
        https: 443,
    };
    // optional: spawn a second server to redirect http requests to this server
    tokio::spawn(redirect_http_to_https(ports, sock_addr));

    log::info!("listening on http://{}", sock_addr);

    let config = RustlsConfig::from_pem_file(PathBuf::from(opt.cert), PathBuf::from(opt.key))
        .await
        .unwrap();

    let alpha_config = config.clone();

    tokio::task::spawn(async move {
        let mut sock_addr = sock_addr.clone();
        sock_addr.set_port(3001);
        let listener = TcpListener::bind(sock_addr).await.unwrap();
        log::info!("listening on http://{}", sock_addr);

        // a https site cannot access http api endpoints
        // too lazy to make the api https
        // axum_server::bind_rustls(sock_addr, alpha_config)
        //     .serve(alpha_app.into_make_service())
        //     .await
        //     .unwrap();
        axum::serve(listener, alpha_app).await.unwrap();
    });

    axum_server::bind_rustls(sock_addr, config.clone())
        .serve(landing_app.into_make_service())
        .await
        .unwrap();

    // let mut sock_addr = sock_addr.clone();
    // sock_addr.set_port(3000);

    // axum_server::bind_rustls(sock_addr, config)
    //     .serve(alpha_app.into_make_service())
    //     .await
    //     .unwrap();

    // axum::Server::bind(&sock_addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .expect("Unable to start server");
}
#[derive(Clone, Copy)]
struct Ports {
    http: u16,
    https: u16,
}

async fn redirect_http_to_https(ports: Ports, sock_addr: SocketAddr) {
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
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let listener = tokio::net::TcpListener::bind(sock_addr).await.unwrap();
    // tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, redirect.into_make_service())
        .await
        .unwrap();
    // .serve(redirect.into_make_service())
    // .await
    // .unwrap();
}
