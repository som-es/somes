use serde::{Deserialize, Serialize};

/// `LoginInfo` is sent by the client and received by the server at login of a user.
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct LoginInfo {
    pub username_or_email: String,
    pub password: String,
}

/// `SignUpInfo` is sent by the client and received by the server at registering a new user.
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct SignUpInfo {
    pub email: String,
    pub username: String,
    pub password: String,
}

/// `UserInfo` contains user specific data.
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct UserInfo {
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct JWTInfo {
    pub access_token: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Unit;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct VerificationIDInfo {
    pub verify_id: String,
}
