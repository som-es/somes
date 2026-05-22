pub mod absences;
pub mod activity;
pub mod age;
pub mod call_to_orders;
pub mod complexity;
pub mod division_accuracy_score;
pub mod error;
pub mod filtering;
pub mod political_orientation;
pub mod session_activity;
pub mod speeches;

pub(crate) fn legislative_period_rank(period: Option<&str>) -> &str {
    period.unwrap_or("")
}

#[cfg(test)]
pub(crate) mod test_db {
    use std::process;
    use std::str::FromStr;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    use once_cell::sync::Lazy;
    use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
    use sqlx::{Connection, Executor, PgConnection, PgPool};

    static DB_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static DB_LOCK: Lazy<Arc<tokio::sync::Mutex<()>>> =
        Lazy::new(|| Arc::new(tokio::sync::Mutex::new(())));
    const STATISTICS_FIXTURE: &str = include_str!("tests/fixtures/statistics_base.sql");

    pub(crate) struct StatisticsTestDb {
        pool: PgPool,
        master_url: String,
        db_name: String,
        _guard: tokio::sync::OwnedMutexGuard<()>,
    }

    impl StatisticsTestDb {
        pub(crate) fn pool(&self) -> &PgPool {
            &self.pool
        }
    }

    impl Drop for StatisticsTestDb {
        fn drop(&mut self) {
            let master_url = self.master_url.clone();
            let db_name = self.db_name.clone();

            let _ = std::thread::spawn(move || {
                let Ok(runtime) = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                else {
                    return;
                };

                runtime.block_on(async move {
                    if let Ok(mut master) = PgConnection::connect(&master_url).await {
                        let _ = master
                            .execute(
                                format!("DROP DATABASE IF EXISTS {db_name:?} WITH (FORCE)")
                                    .as_str(),
                            )
                            .await;
                    }
                });
            })
            .join();
        }
    }

    pub(crate) async fn statistics_test_db(test_name: &str) -> StatisticsTestDb {
        let guard = DB_LOCK.clone().lock_owned().await;
        let _ = dotenvy::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/../.env"));
        let master_url =
            dotenvy::var("SOMES_TEST_DB_URL").expect("SOMES_TEST_DB_URL must be set for DB tests");
        let counter = DB_COUNTER.fetch_add(1, Ordering::SeqCst);
        let safe_test_name: String = test_name
            .chars()
            .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
            .collect();
        let safe_test_name = safe_test_name
            .chars()
            .take(24)
            .collect::<String>()
            .trim_end_matches('_')
            .to_string();
        let db_name = format!(
            "somes_statistics_test_{}_{}_{}",
            process::id(),
            counter,
            safe_test_name
        );

        let mut master = PgConnection::connect(&master_url)
            .await
            .expect("failed to connect to SOMES_TEST_DB_URL");
        master
            .execute(format!("DROP DATABASE IF EXISTS {db_name:?} WITH (FORCE)").as_str())
            .await
            .expect("failed to drop stale statistics test database");
        master
            .execute(format!("CREATE DATABASE {db_name:?}").as_str())
            .await
            .expect("failed to create statistics test database");

        let test_options = PgConnectOptions::from_str(&master_url)
            .expect("failed to parse SOMES_TEST_DB_URL")
            .database(&db_name);
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(test_options)
            .await
            .expect("failed to connect to statistics test database");

        sqlx::raw_sql(STATISTICS_FIXTURE)
            .execute(&pool)
            .await
            .expect("failed to apply statistics test fixture");

        StatisticsTestDb {
            pool,
            master_url,
            db_name,
            _guard: guard,
        }
    }
}

pub use absences::*;
pub use activity::*;
pub use age::*;
pub use call_to_orders::*;
pub use complexity::*;
pub use division_accuracy_score::*;
pub use error::*;
pub use filtering::*;
pub use political_orientation::*;
pub use session_activity::*;
pub use speeches::*;
