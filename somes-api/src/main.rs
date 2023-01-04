use std::net::SocketAddr;

use redis::AsyncCommands;
use somes_api::{initial_startup, model::NewUser, server, DB_PATH, SQL_SCHEMA_PATH};

#[tokio::main]
async fn main() {
    initial_startup(DB_PATH, SQL_SCHEMA_PATH);

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await.unwrap();

    let new_user = NewUser {
        username: Default::default(),
        email: Default::default(),
        password_hash: Default::default(),
    };
    con.set::<_, _, ()>("hi", new_user).await.unwrap();
    let value = con.get::<_, NewUser>("hi").await.unwrap();
    println!("value: {value:?}");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    server::serve(addr).await;
}
