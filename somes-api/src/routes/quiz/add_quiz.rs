use axum::Json;
use reqwest::StatusCode;
use sqlx::{query, PgPool};

use crate::{jwt::Claims, GenericErrorResponse, PgPoolConnection};

pub struct QuizQuestion {
    pub question: String,
    pub answer1: String,
    pub answer2: String,
    pub answer3: String,
    pub answer4: String,
}

pub struct Quiz {
    pub title: String,
    pub description: String,
    pub questions: Vec<QuizQuestion>,
}

pub async fn add_quiz_handler(pg: &PgPool, user_id: i32, quiz: Quiz) -> crate::Result<()> {
    let is_admin = query!("select is_admin from somes_user where id = $1", user_id)
        .fetch_one(pg)
        .await
        .map_err(|e| GenericErrorResponse::DbSelectFailure(Some(e)))?;
    if !is_admin.is_admin {
        return Err(GenericErrorResponse::Custom((
            StatusCode::UNAUTHORIZED,
            "missing permissions",
        )));
    }

    let id = query!(
        "insert into quiz (title, description) values ($1, $2) returning id",
        quiz.title,
        quiz.description
    )
    .fetch_one(pg)
    .await
    .map_err(|e| GenericErrorResponse::DbSelectFailure(Some(e)))?;

    for question in quiz.questions {
        query!("insert into quiz_questions (quiz_id, answer1, answer2, answer3, answer4) values ($1, $2, $3, $4, $5)", id.id, question.answer1, question.answer2, question.answer3, question.answer4).execute(pg).await
            .map_err(|e| GenericErrorResponse::DbSelectFailure(Some(e)))?;
    }

    Ok(())
}

pub async fn add_quiz(
    claims: Claims,
    Json(quiz): Json<Quiz>,
    PgPoolConnection(pg): PgPoolConnection,
) -> crate::Result<Json<()>> {
    add_quiz_handler(&pg, claims.id, quiz).await.map(Json)
}
