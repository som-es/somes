use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateDelegateQuestion {
    pub subject: String,
    pub body: String,
    pub eurovoc_topic_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DelegateQuestionCreated {
    pub id: i64,
    pub delivery: QuestionDelivery,
    pub recipient_name: String,
    pub status: String,
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum QuestionDelivery {
    Delegate,
    Party,
}

impl QuestionDelivery {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Delegate => "delegate",
            Self::Party => "party",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DelegateQuestionRecipient {
    pub delivery: QuestionDelivery,
    pub recipient_name: String,
}

#[derive(Debug, Serialize)]
pub struct PublicDelegateQuestion {
    pub delegate_id: i32,
    pub subject: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub answers: Vec<PublicDelegateQuestionAnswer>,
}

#[derive(Debug, Serialize)]
pub struct PublicDelegateQuestionAnswer {
    pub body: String,
    pub received_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminDelegateQuestion {
    pub id: i64,
    pub user_id: i32,
    pub delegate_id: i32,
    pub delegate_name: String,
    pub recipient_email: String,
    pub recipient_kind: String,
    pub recipient_name: String,
    pub subject: String,
    pub body: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

pub(super) struct DelegateContact {
    pub name: String,
    pub recipient_name: String,
    pub recipient_email: String,
    pub delivery: QuestionDelivery,
}
