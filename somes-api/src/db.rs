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
use diesel::{Connection, PgConnection};
use redis::AsyncCommands;
use reqwest::StatusCode;

use crate::DATABASE_URL;

use self::model::NewUser;

pub fn establish_connection() -> PgConnection {
    PgConnection::establish(DATABASE_URL).expect("Can't establish database conntection.")
}

pub struct RedisConnection(pub redis::aio::Connection);

pub async fn extract_to_be_verified_from_redis(
    redis_con: &mut redis::aio::Connection,
) -> Vec<NewUser> {
    let keys: Vec<String> = redis_con.keys("*").await.unwrap();

    let mut values = Vec::<NewUser>::with_capacity(keys.len());

    for key in keys {
        if let Ok(value) = redis_con.get(key).await {
            values.push(value);
        }
    }
    values
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
        let conn = pool.get_async_connection().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

fn internal_error<E>(err: E) -> (StatusCode, String)
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
