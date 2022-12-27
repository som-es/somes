use std::net::SocketAddr;

use somes_api::{initial_startup, server, DB_PATH, SQL_SCHEMA_PATH};

#[tokio::main]
async fn main() {
    initial_startup(DB_PATH, SQL_SCHEMA_PATH);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    server::serve(addr).await;
}
