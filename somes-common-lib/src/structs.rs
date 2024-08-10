use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// 'ResetPasswordInfo' is used to send a reset password request to the server.
#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct ResetPasswordInfo {
    pub username_or_email: String,
}

/// 'NewPasswordInfo' is used to transmit a new password to the server.
#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct NewPasswordInfo {
    pub password: String,
}

/// `LoginInfo` is sent by the client and received by the server at login of a user.
#[derive(ToSchema, IntoParams, Debug, Deserialize, Serialize, Default, Clone)]
pub struct LoginInfo {
    pub username_or_email: String,
    pub password: String,
}

/// `SignUpInfo` is sent by the client and received by the server at registering a new user.
#[derive(ToSchema, IntoParams, Debug, Deserialize, Serialize, Default, Clone)]
pub struct SignUpInfo {
    pub email: String,
    pub username: String,
    pub password: String,
}

/// `UserInfo` contains user specific data.
#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct UserInfo {
    pub username: String,
    pub email: String,
}

#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct JWTInfo {
    pub access_token: String,
}

#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone, Copy)]
pub struct Unit;

#[derive(ToSchema, IntoParams, Debug, Deserialize, Serialize, Default, Clone)]
pub struct VerificationIDInfo {
    pub id: String,
}

#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct SaveEmailInfo {
    pub email: String,
}

#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct AskQuestion {
    pub delegate_id: i32,
    pub title: String,
    pub body: String,
}

#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct Question {
    pub question_id: i32,
    pub issuer_id: i32, // user
    pub created_on: NaiveDateTime,
    pub delegate_id: i32,
    pub title: String,
    pub body: String,
    pub response: Option<String>,
    pub responded_on: Option<NaiveDateTime>,
    pub editable: bool, // 100% false if the delegate does not have a somes account
    pub last_edited_on: Option<NaiveDateTime>,
    pub visible: bool, // maybe if an admin flags a question as inappropriate?
}

#[derive(IntoParams, ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct DateRange {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

#[derive(IntoParams, ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct QuestionFilter {
    pub page: i32,
    pub filter_text: Option<String>,
    pub filter_delegate: Option<i32>,
    pub filter_party: Option<i32>,
    pub filter_date_range: Option<DateRange>,
    pub filter_topics: Option<Vec<String>>, // maybe
}

#[derive(IntoParams, ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct DelegateById {
    pub delegate_id: i32,
}

#[derive(IntoParams, ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct InterestShare {
    pub topic: String,
    pub total_share: f32,
    pub self_share: f32,
}

#[derive(IntoParams, ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct Page {
    pub page: i64,
}

#[derive(Default, IntoParams, ToSchema, Debug, Deserialize, Serialize, Clone)]
pub struct LegisInitFilter {
    pub accepted: Option<String>,
    pub is_named_vote: Option<bool>,
    pub simple_majority: Option<bool>,
    pub legis_period: Option<String>,
}
