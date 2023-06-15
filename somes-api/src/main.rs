use simple_logger::SimpleLogger;
use somes_api::{jwt::KEYS, server};

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    // if a JWT_SECRET is not present, crash the application
    let _ = &KEYS.decoding;

    // this addr is also used in email
    //let addr = "127.0.0.1:3000".parse().unwrap();
    let addr = "172.20.10.2:3000".parse().unwrap();
    server::serve(addr).await;
}
