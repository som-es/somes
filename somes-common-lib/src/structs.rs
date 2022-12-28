use serde::{Deserialize, Serialize};

/// `LoginInfo` is sent by the client and received by the server at login of a user.
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

/// `SignUpInfo` is sent by the client and received by the server at registering a new user.
#[derive(Debug, Deserialize, Serialize)]
pub struct SignUpInfo {
    pub email: String,
    pub username: String,
    pub password: String,
}

/// `UserInfo` contains user specific data.
#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JWTInfo {
    access_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VerificationIDInfo {
    pub verify_id: String,
}
