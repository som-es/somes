pub mod model;
pub mod operations;
pub mod schema;

use std::{
    fmt::Display,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};

// use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};
use redis::{aio::MultiplexedConnection, AsyncCommands, Client, Commands};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::PgPool;

use crate::{server::AppState, PostgresPool};

use self::model::NewUser;

#[cfg(test)]
pub fn establish_connection() -> diesel::PgConnection {
    use diesel::PgConnection;

    <PgConnection as diesel::Connection>::establish(crate::USR_DATABASE_URL)
        .expect("Can't establish database conntection.")
}

pub async fn get_json_cache<T: DeserializeOwned>(
    redis_client: &mut MultiplexedConnection,
    key: &str,
) -> Option<T> {
    #[cfg(debug_assertions)]
    {
        None
    }
    #[cfg(not(debug_assertions))]
    serde_json::from_str(&redis_client.get::<&str, String>(key).await.ok()?).ok()
}

pub async fn set_json_cache_secs<T: Serialize>(
    redis_client: &mut MultiplexedConnection,
    key: &str,
    value: &T,
    seconds: i64,
) -> Option<()> {
    redis_client
        .set(key, serde_json::to_string(value).ok()?)
        .await
        .ok()?;
    redis_client.expire::<_, ()>(key, seconds).await.ok()?;
    Some(())
}

pub async fn set_json_cache<T: Serialize>(
    redis_client: &mut MultiplexedConnection,
    key: &str,
    value: &T,
) -> Option<()> {
    set_json_cache_secs(redis_client, key, value, 1200).await
}

pub struct RedisConnection(pub redis::aio::MultiplexedConnection);

pub async fn extract_to_be_verified_from_redis(
    redis_con: &mut redis::aio::MultiplexedConnection,
) -> redis::RedisResult<Vec<NewUser>> {
    let keys: Vec<String> = redis_con.keys("*").await?;

    let mut values = Vec::<NewUser>::with_capacity(keys.len());

    for key in keys {
        if let Ok(value) = redis_con.get(key).await {
            values.push(value);
        }
    }
    Ok(values)
}

#[async_trait]
impl<S> FromRequestParts<S> for RedisConnection
where
    redis::Client: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = redis::Client::from_ref(state);
        let conn = pool
            .get_multiplexed_async_connection()
            .await
            .map_err(internal_error)?;

        Ok(Self(conn))
    }
}

pub struct DataserviceDbConnection(pub deadpool_diesel::postgres::Object);

// pub struct DataserviceDbConnection(
//     // pub bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
//     pub deadpool_diesel::postgres::Object,
// );

// #[async_trait]
// impl<S> FromRequestParts<S> for DataserviceDbConnection
// where
//     S: Send + Sync,
//     PostgresPool: FromRef<S>,
// {
//     type Rejection = (StatusCode, String);

//     async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         let pool = PostgresPool::from_ref(state);
//         let conn = pool.get().await.map_err(internal_error)?;

//         Ok(Self(conn))
//     }
// }

// #[async_trait]
// impl FromRequestParts<AppState> for DataserviceDbConnection {
//     type Rejection = (StatusCode, String);

//     async fn from_request_parts(
//         _parts: &mut Parts,
//         state: &AppState,
//     ) -> Result<Self, Self::Rejection> {
//         let conn = state.somes_db_pool.get().await.map_err(internal_error)?;
//         Ok(Self(conn))
//     }
// }

#[async_trait]
impl FromRequestParts<AppState> for DataserviceDbConnection {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let conn = state
            .dataservice_db_pool
            .get()
            .await
            .map_err(internal_error)?;
        Ok(Self(conn))
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(app_state: &AppState) -> PgPool {
        app_state.dataservice_sqlx_pool.clone()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for PgPoolConnection
where
    S: Send + Sync,
    PgPool: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);
        Ok(Self(pool))
    }
}

pub struct PgPoolConnection(pub PgPool);

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[derive(Debug)]
pub enum CreateDBError {
    ReadDBFile,
    SQLNotFound(&'static str),
    Sqlite3CommandExec,
}

impl Display for CreateDBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateDBError::SQLNotFound(file) => write!(f, "The (sql) file '{file}' does not exist."),
            CreateDBError::Sqlite3CommandExec => write!(f, "The execution of the sqlite3 command was not successful. -> Database was not created."),
            CreateDBError::ReadDBFile => write!(f, "Could not check whether the created database is empty."),
        }
    }
}

#[macro_export]
macro_rules! interact {
    ($con:ident, $code: expr) => {
        $con.interact(|$con| $code).await
    };
}

/// Creates an empty database if it does not exist.
pub fn create_db_if_not_exists(
    db_path: impl AsRef<Path>,
    sql_db_path_str: &'static str,
) -> Result<(), CreateDBError> {
    if db_path.as_ref().exists() {
        // `establish_connection` creates a new (empty) database if none exists.
        // This checks if the database is empty. If so => try to create a new database
        if !std::fs::read(db_path.as_ref())
            .map_err(|_| CreateDBError::ReadDBFile)?
            .is_empty()
        {
            return Ok(());
        }
    }

    if !Path::new(sql_db_path_str).exists() {
        return Err(CreateDBError::SQLNotFound(sql_db_path_str));
    }

    create_db(db_path, sql_db_path_str)?;

    Ok(())
}

pub fn create_db(
    db_path: impl AsRef<Path>,
    sql_db_path_str: &'static str,
) -> Result<(), CreateDBError> {
    // runs the sqlite3 command with an input §channel".
    let mut child = Command::new("sqlite3")
        .arg(db_path.as_ref().to_str().unwrap())
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|_| CreateDBError::Sqlite3CommandExec)?;

    // reads the sql code from the provided path
    let sql_code = std::fs::read_to_string(sql_db_path_str)
        .map_err(|_| CreateDBError::SQLNotFound(sql_db_path_str))?;

    // writes the sql code to the stdin of the sqlite3 command
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(sql_code.as_bytes())
        .map_err(|_| CreateDBError::Sqlite3CommandExec)?;

    Ok(())
}
