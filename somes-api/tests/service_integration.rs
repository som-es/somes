use sqlx::postgres::PgPoolOptions;

fn required_env(name: &str) -> String {
    std::env::var(name)
        .unwrap_or_else(|_| panic!("{name} must be set for service integration tests"))
}

#[tokio::test]
#[ignore = "requires a disposable Postgres instance"]
async fn postgres_healthcheck_can_run_simple_query() {
    let database_url = required_env("SOMES_INTEGRATION_DATABASE_URL");
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .unwrap();

    let value = sqlx::query_scalar::<_, i64>("SELECT 1")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(value, 1);
}

#[tokio::test]
#[ignore = "requires a disposable Redis instance"]
async fn redis_can_round_trip_a_namespaced_key() {
    use redis::AsyncCommands;

    let redis_url = required_env("SOMES_INTEGRATION_REDIS_URL");
    let client = redis::Client::open(redis_url).unwrap();
    let mut con = client.get_multiplexed_async_connection().await.unwrap();
    let key = format!("somes:test:{}", uuid::Uuid::new_v4());

    con.set_ex::<_, _, ()>(&key, "ok", 30).await.unwrap();
    let value: String = con.get(&key).await.unwrap();
    con.unlink::<_, ()>(&key).await.unwrap();

    assert_eq!(value, "ok");
}

#[tokio::test]
#[ignore = "requires a disposable Meilisearch instance"]
async fn meilisearch_healthcheck_reports_available() {
    let meilisearch_url = required_env("SOMES_INTEGRATION_MEILISEARCH_URL");
    let meilisearch_key = std::env::var("SOMES_INTEGRATION_MEILISEARCH_KEY").ok();
    let client =
        meilisearch_sdk::client::Client::new(meilisearch_url, meilisearch_key.as_deref()).unwrap();

    let health = client.health().await.unwrap();

    assert_eq!(health.status, "available");
}
