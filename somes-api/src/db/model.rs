use std::str::from_utf8;

use diesel::prelude::*;
use redis::{ErrorKind, FromRedisValue, RedisError, ToRedisArgs, Value};
use serde::{Deserialize, Serialize};

use super::schema::users;

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl ToRedisArgs for NewUser {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(serde_json::to_string(self).unwrap().as_bytes())
    }
}

impl FromRedisValue for NewUser {
    fn from_redis_value(val: &redis::Value) -> redis::RedisResult<Self> {
        match val {
            Value::Data(ref bytes) => Ok(serde_json::from_str(&from_utf8(bytes)?)
                .map_err(|_| RedisError::from((ErrorKind::TypeError, "User cannot be parsed.")))?),
            _ => Err(RedisError::from((
                ErrorKind::TypeError,
                "User cannot be parsed.",
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use redis::AsyncCommands;

    use crate::model::NewUser;

    #[tokio::test]
    async fn test_user_insert() {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_async_connection().await.unwrap();

        let new_user = NewUser {
            username: "name".to_string(),
            email: "email".to_string(),
            password_hash: "hash".to_string(),
        };

        con.set::<_, _, ()>("hi", &new_user).await.unwrap();

        let value = con.get::<_, NewUser>("hi").await.unwrap();

        assert_eq!(value.email, new_user.email);
        assert_eq!(value.username, new_user.username);
        assert_eq!(value.password_hash, new_user.password_hash);
    }
}
