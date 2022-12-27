use crate::db::create_db_if_not_exists;

pub fn initial_startup(db_path: &'static str, sql_db_path: &'static str) {
    // production database
    if let Err(e) = create_db_if_not_exists(db_path, sql_db_path) {
        panic!("Error: {e}");
    }
}
