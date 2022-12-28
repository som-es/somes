pub mod model;
pub mod operations;
pub mod schema;

use std::{
    fmt::Display,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use diesel::{Connection, SqliteConnection};

use crate::DB_PATH;

#[cfg(test)]
pub fn establish_test_connection(db_path: &str) -> SqliteConnection {
    SqliteConnection::establish(db_path).expect("Can't establish database conntection.")
}

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish(DB_PATH).expect("Can't establish database conntection.")
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

#[cfg(test)]
pub mod test_db {
    use diesel::SqliteConnection;

    use crate::{create_db_if_not_exists, establish_test_connection};

    /// A handle to the testing database
    #[derive(Debug)]
    pub struct DBHandle {
        db_path: String,
    }

    impl DBHandle {
        pub fn establish_connection(&self) -> SqliteConnection {
            establish_test_connection(&self.db_path)
        }
    }

    impl Drop for DBHandle {
        fn drop(&mut self) {
            std::fs::remove_file(&self.db_path).unwrap();
        }
    }

    #[must_use = "This handle deletes the old database on drop. Not using this 'instantly' deletes the testing database."]
    pub fn create_handle(id: u64) -> DBHandle {
        let db_handle = DBHandle {
            db_path: format!("{}{id}", crate::TEST_DB_PATH),
        };

        create_db_if_not_exists(&db_handle.db_path, crate::SQL_SCHEMA_PATH).unwrap();
        db_handle
    }

    #[macro_export]
    macro_rules! id {
        () => {{
            let id = concat!(file!(), line!(), column!());
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};

            let mut s = DefaultHasher::new();
            id.hash(&mut s);
            s.finish()
        }};
    }
}
