use diesel::prelude::*;

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}