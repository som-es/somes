use chrono::{NaiveDate, NaiveDateTime};
use common_scrapes::Voting;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(ToSchema, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Delegate {
    pub id: i32,
    pub name: String,
    pub party: Option<String>,
    pub current_party: Option<String>,
    pub image_url: Option<String>,
    pub constituency: Option<String>,
    pub council: Option<String>,
    pub seat_row: Option<i32>,
    pub seat_col: Option<i32>,
    pub gender: Option<String>,
    pub is_active: Option<bool>,
    pub birthdate: Option<NaiveDate>,
    pub active_since: NaiveDate,
    pub divisions: Option<Vec<String>>,
    pub primary_mandate: Option<String>,
    pub active_mandates: Option<Vec<String>>,
}

#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct DelegateFavo {
    pub delegate_id: i32,
}

#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct LegisInitFavo {
    pub vote_result_id: i32,
}

#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct Absence {
    pub date: NaiveDate,
    pub inr: i32,
    pub gp: String,
    pub plenary_session_id: i32,
    pub missed_legis_init_ids: Option<Vec<i32>>,
}

#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct AbsencesWithMaxPage {
    pub speeches: Vec<Absence>,
    pub entry_count: i64,
    pub max_page: i64,
}

#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct NamedVote {
    pub infavor: Option<bool>,
    pub was_absent: Option<bool>,
    pub legis_init_id: i32,
    pub named_vote_info_id: i32,
    pub date: NaiveDate,
}

#[derive(ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct StanceTopicScore {
    pub topic: String,
    pub score: f64,
}

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
    pub email: String,
    pub password: Option<String>,
    pub hash_email: Option<bool>,
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
pub struct SendMailInfo {
    pub send_new_vote_results_mails: bool,
    pub send_new_delegate_activity_mails: bool,
    pub send_new_ministrial_prop_mails: bool,
}

#[derive(IntoParams, ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct DelegateByIdAndPage {
    pub delegate_id: i32,
    pub page: i64,
}

#[derive(IntoParams, ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct InterestShare {
    pub topic: String,
    pub occurences: u32,
    pub total_share: f32,
    pub self_share: f32,
}

#[derive(IntoParams, ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct Mandate {
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub name: String,
}

#[derive(IntoParams, ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct GeneralDelegateInfo {
    pub mandates: Vec<Mandate>,
    pub interests: Vec<InterestShare>,
    pub detailed_interests: Vec<InterestShare>,
    pub delegate_qa: Vec<DelegateQA>,
    pub political_position: Option<PoliticalPosition>,
    pub absences: Vec<Absence>,
    pub named_votes: Vec<NamedVote>,
    pub left_right_stances: Vec<StanceTopicScore>,
    pub stances: Vec<StanceTopicScore>,
}

#[derive(IntoParams, ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct DelegateQA {
    pub question: String,
    pub answer: String,
}

#[derive(IntoParams, ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct PoliticalPosition {
    pub delegate_id: i32,
    pub is_left: f64,
    pub is_not_left: f64,
    pub is_liberal: f64,
    pub is_not_liberal: f64,
    pub neutral_count: i32,
}

#[derive(IntoParams, ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct PoliticalAnswer {
    pub delegate_id: i32,
    pub answer: String,
    pub question: String,
}

#[derive(IntoParams, ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct Page {
    pub page: i64,
}

#[derive(IntoParams, ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct VoteResultById {
    pub id: i32,
}

#[derive(IntoParams, ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct SearchQuery {
    pub search: String,
}

#[derive(IntoParams, ToSchema, Debug, Deserialize, Serialize, Default, Clone)]
pub struct Date {
    pub at: NaiveDate,
}

#[derive(Default, IntoParams, ToSchema, Debug, Deserialize, Serialize, Clone)]
pub struct LegisPeriodFilter {
    pub legis_period: String,
}

#[derive(Default, IntoParams, ToSchema, Debug, Deserialize, Serialize, Clone)]
pub struct PartyVote {
    pub infavor: bool,
    pub party: String,
}

#[derive(Default, IntoParams, ToSchema, Debug, Deserialize, Serialize, Clone)]

pub struct LegisInitFilter {
    pub accepted: Option<String>,
    pub is_named_vote: Option<bool>,
    pub simple_majority: Option<bool>,
    pub legis_period: Option<String>,
    pub is_law: Option<bool>,
    pub vote_type: Option<Voting>,
    pub topics: Option<Vec<String>>,
    pub party_votes: Option<Vec<PartyVote>>,
}

#[derive(Default, IntoParams, ToSchema, Debug, Deserialize, Serialize, Clone)]
pub struct GovPropFilter {
    pub has_vote_result: Option<bool>,
    pub legis_period: Option<String>,
}

#[derive(Default, IntoParams, ToSchema, Debug, Deserialize, Serialize, Clone)]
pub struct DecreeFilter {
    pub legis_period: Option<String>,
    pub gov_officials: Option<Vec<i32>>
}

#[derive(PartialEq, Eq, IntoParams, ToSchema, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct LegisPeriod {
    pub period: String,
}

#[derive(PartialEq, Eq, IntoParams, ToSchema, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct LegisPeriodGp {
    pub gp: String,
}
