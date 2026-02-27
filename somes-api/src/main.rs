use simple_logger::SimpleLogger;
use somes_api::{email::MAILER, jwt::KEYS, server, IS_PROD};

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
    if *IS_PROD {
        log::info!("Running in production mode");
    } else {
        log::info!("Running in development mode");
    }

    let _ = dotenvy::dotenv().ok();

    // if a JWT_SECRET is not present, crash the application
    let _ = &KEYS.decoding;

    // use MAILER
    let _mailer = &*MAILER;

    // this addr is also used in email
    let addr = somes_api::HOST_ADDR.parse().unwrap();
    server::serve(addr).await;
}
