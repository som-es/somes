use axum::Json;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};

use crate::{jwt::Claims, routes::QuizQuestion, GenericErrorResponse, PgPoolConnection};


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct QuizQuery {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct QuizId {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub questions: Vec<QuizQuestion>,
}

pub async fn get_all_quizzes_handler(pg: &PgPool, user_id: i32) -> crate::Result<Vec<QuizId>> {
    let is_admin = query!("SELECT is_admin FROM somes_user WHERE id = $1", user_id)
        .fetch_one(pg)
        .await
        .map_err(|e| GenericErrorResponse::DbSelectFailure(Some(e)))?;

    if !is_admin.is_admin {
        return Err(GenericErrorResponse::Custom((
            StatusCode::UNAUTHORIZED,
            "missing permissions",
        )));
    }

    let quizzes = query_as!(QuizQuery, "SELECT id, title,  description FROM quiz")
        .fetch_all(pg)
        .await
        .map_err(|e| GenericErrorResponse::DbSelectFailure(Some(e)))?;

    let mut quizzes_with_questions = Vec::new();
    for quiz_query in quizzes {
        let questions = query_as!(
            QuizQuestion,
            "SELECT question, answer1, answer2, answer3, answer4, correct_answer FROM quiz_questions WHERE quiz_id = $1",
            quiz_query.id
        )
        .fetch_all(pg)
        .await
        .map_err(|e| GenericErrorResponse::DbSelectFailure(Some(e)))?;

        quizzes_with_questions.push(QuizId {
            id: quiz_query.id,
            title: quiz_query.title,
            description: quiz_query.description,
            questions,
        });
    }

    Ok(quizzes_with_questions)
}

pub async fn get_all_quizzes(
    claims: Claims,
    PgPoolConnection(pg): PgPoolConnection,
) -> crate::Result<Json<Vec<QuizId>>> {
    Ok(Json(get_all_quizzes_handler(&pg, claims.id).await?))
}
