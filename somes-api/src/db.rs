pub mod model;
pub mod schema;

use std::{
    fmt::Display,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use diesel::{Connection, SqliteConnection};

use crate::DB_PATH;

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish(DB_PATH)
        .expect("Can't establish database conntection.")
}

pub enum CreateDBError {
    SQLNotFound(&'static str),
    Sqlite3CommandExec,
}

impl Display for CreateDBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateDBError::SQLNotFound(file) => write!(f, "The (sql) file '{file}' does not exist."),
            CreateDBError::Sqlite3CommandExec => write!(f, "The execution of the sqlite3 command was not successful. -> Database was not created."),
        }
    }
}

/// Creates an empty database if it does not exist.
pub fn create_db_if_not_exists(
    db_path: impl AsRef<Path>,
    sql_db_path_str: &'static str,
) -> Result<(), CreateDBError> {
    if db_path.as_ref().exists() {
        return Ok(());
    }

    if !Path::new(sql_db_path_str).exists() {
        return Err(CreateDBError::SQLNotFound(sql_db_path_str));
    }

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
