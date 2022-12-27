use std::net::SocketAddr;

use somes_api::{
    initial_startup, server, testing_initial_startup, DB_PATH, SQL_SCHEMA_PATH, TEST_DB_PATH,
};

#[tokio::main]
async fn main() {
    testing_initial_startup(TEST_DB_PATH, SQL_SCHEMA_PATH);
    initial_startup(DB_PATH, SQL_SCHEMA_PATH);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    server::serve(addr).await;
}
