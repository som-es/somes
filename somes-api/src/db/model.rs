use diesel::prelude::*;
use redis::{ToRedisArgs, FromRedisValue};
use serde::{Serialize, Deserialize};

use super::schema::user;

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}


impl ToRedisArgs for NewUser {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite 
    {
        ToRedisArgs::make_arg_vec(&[&self.username, &self.email, &self.password_hash], out)
    }
}

impl FromRedisValue for NewUser {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        todo!()
    }
}