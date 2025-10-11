use dotenvy_macro::dotenv;
use sqlx::postgres::PgPoolOptions;

pub const DATASERVICE_URL: &str = dotenv!("DATASERVICE_URL");

#[tokio::main]
async fn main() {
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=build.rs");
    let dataservice_sqlx_pool = PgPoolOptions::new()
        // pool sizes
        .max_connections(2)
        .connect(DATASERVICE_URL)
        .await
        .unwrap();

    views::create_views(&dataservice_sqlx_pool).await.unwrap();
}
