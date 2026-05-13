use dotenvy_macro::dotenv;
use sqlx::postgres::PgPoolOptions;

pub const DATASERVICE_URL: &str = dotenv!("DATASERVICE_URL");

#[tokio::main]
async fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=UPDATE_VIEWS");

    let update_views = std::env::var("UPDATE_VIEWS").unwrap_or_else(|_| "false".to_string());
    if update_views != "true" {
        return;
    }

    let dataservice_sqlx_pool = PgPoolOptions::new()
        // pool sizes
        .max_connections(2)
        .connect(DATASERVICE_URL)
        .await
        .unwrap();

    let mut tx = dataservice_sqlx_pool.begin().await.unwrap();
    views::create_composite_types(&mut tx).await.unwrap();
    views::create_views(&mut tx).await.unwrap();
    tx.commit().await.unwrap();
}
