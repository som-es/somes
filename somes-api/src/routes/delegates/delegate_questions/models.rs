use chrono::{DateTime, Utc};
use common_scrapes::language::Language;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub struct DelegateQuestionQuery {
    #[serde(default)]
    pub language: Language,
}

#[derive(Debug, Deserialize)]
pub struct CreateDelegateQuestion {
    pub subject: String,
    pub body: String,
    pub eurovoc_topic_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDelegateQuestion {
    pub subject: Option<String>,
    pub body: Option<String>,
    pub eurovoc_topic_ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct DelegateQuestionCreated {
    pub id: i64,
    pub delivery: QuestionDelivery,
    pub recipient_name: String,
    pub status: String,
    pub topics: Vec<DelegateQuestionTopic>,
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

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct DelegateQuestionTopic {
    pub id: String,
    pub topic: String,
}

#[derive(Debug, Serialize)]
pub struct PublicDelegateQuestion {
    pub delegate_id: i32,
    pub subject: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub topics: Vec<DelegateQuestionTopic>,
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
    pub topics: Vec<DelegateQuestionTopic>,
}

pub(super) struct DelegateContact {
    pub name: String,
    pub recipient_name: String,
    pub recipient_email: String,
    pub delivery: QuestionDelivery,
}

#[cfg(test)]
mod tests {
    use super::DelegateQuestionQuery;
    use axum::{extract::Query, http::Uri};
    use common_scrapes::language::Language;

    fn language_of(query: &str) -> Language {
        let uri: Uri = format!("/questions?{query}").parse().unwrap();
        Query::<DelegateQuestionQuery>::try_from_uri(&uri)
            .expect("language query must deserialize")
            .0
            .language
    }

    #[test]
    fn language_defaults_to_german() {
        assert_eq!(language_of(""), Language::De);
        assert_eq!(language_of("language=de"), Language::De);
        assert_eq!(language_of("language=en"), Language::En);
        assert_eq!(language_of("language=fr"), Language::Fr);
    }
}
