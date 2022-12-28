use diesel::prelude::*;

use super::schema::user;

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}
