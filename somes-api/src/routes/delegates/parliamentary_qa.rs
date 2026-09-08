use combx::models::*;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Deserialize, Serialize)]
pub struct ParliamentInquiryResponseWithMaxPage {
    pub question_entries: Vec<ParliamentInquiryResponse>,
    pub entry_count: i64,
    pub max_page: i64,
}

impl ParliamentInquiryResponseWithMaxPage {
    fn new(
        question_entries: Vec<ParliamentInquiryResponse>,
        page_elements: i64,
        all_entries_count: i64,
    ) -> Self {
        let max_page = (all_entries_count as f64 / page_elements as f64).ceil() as i64;
        Self {
            question_entries,
            entry_count: page_elements,
            max_page,
        }
    }
}

pub async fn extract_parliamentary_questions(
    delegate_id: i32,
    page: i64,
    page_elements: i64,
    pg_pool: &PgPool,
) -> sqlx::Result<ParliamentInquiryResponseWithMaxPage> {
    let offset = (page - 1) * page_elements;

    let rows = sqlx::query_as!(
        ParliamentInquiryResponse,
        r#"
        SELECT *
        FROM pqa_composite_questions
        WHERE $1 = ANY((("question: ParliamentInquiry").data).issuer_ids)
        ORDER BY (("question: ParliamentInquiry").data).raw_data_created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        delegate_id as i64,
        page_elements,
        offset,
    )
    .fetch_all(pg_pool)
    .await?;

    let all_entries_count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM pqa_composite_questions
        WHERE $1 = ANY((("question: ParliamentInquiry").data).issuer_ids)
        "#,
        delegate_id as i64,
    )
    .fetch_one(pg_pool)
    .await?
    .unwrap_or(0);

    Ok(ParliamentInquiryResponseWithMaxPage::new(
        rows,
        page_elements,
        all_entries_count,
    ))
}

pub async fn extract_parliamentary_answers(
    delegate_id: i32,
    page: i64,
    page_elements: i64,
    pg_pool: &PgPool,
) -> sqlx::Result<ParliamentInquiryResponseWithMaxPage> {
    let offset = (page - 1) * page_elements;

    let rows = sqlx::query_as!(
        ParliamentInquiryResponse,
        r#"
        SELECT *
        FROM pqa_composite_questions
        WHERE $1 = ANY((("question: ParliamentInquiry").data).receiver_ids)
        ORDER BY (("question: ParliamentInquiry").data).raw_data_created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        delegate_id as i64,
        page_elements,
        offset,
    )
    .fetch_all(pg_pool)
    .await?;

    let all_entries_count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM pqa_composite_questions
        WHERE $1 = ANY((("question: ParliamentInquiry").data).receiver_ids)
        "#,
        delegate_id as i64,
    )
    .fetch_one(pg_pool)
    .await?
    .unwrap_or(0);

    Ok(ParliamentInquiryResponseWithMaxPage::new(
        rows,
        page_elements,
        all_entries_count,
    ))
}

pub async fn extract_questions_with_ai_content(
    pg_pool: &PgPool,
) -> sqlx::Result<Vec<ParliamentInquiryResponse>> {
    sqlx::query_as!(
        ParliamentInquiryResponse,
        r#"
        SELECT *
        FROM pqa_composite_questions
        WHERE (("question: ParliamentInquiry").ai_question) IS NOT NULL
          AND EXISTS (
              SELECT 1
              FROM unnest(("answer: Vec<ParliamentAnswer>")) AS ans
              WHERE (ans).ai_answer IS NOT NULL
          )
        order by random()
        limit 1
        "#,
    )
    .fetch_all(pg_pool)
    .await
}
#[cfg(test)]
mod tests {
    use crate::routes::{extract_parliamentary_answers, extract_questions_with_ai_content};
    use combx::{ParliamentInquiryResponse, connect_pg};

    #[tokio::test]
    pub async fn test_extract_parliamentary_answers_for_gov_official() {
        let pg = connect_pg().await;
        let res = extract_parliamentary_answers(20445, 1, 20, &pg)
            .await
            .unwrap();
        println!("total: {}, max page: {}", res.entry_count, res.max_page);
        println!("returned: {}", res.question_entries.len());

        for question_entry in &res.question_entries {
            let answers = question_entry.answer.clone().unwrap_or_default();
            if !answers.is_empty() {
                println!("answers: {answers:?}");
                // let file = std::fs::File::create("tests/question_response.json").unwrap();
                // serde_json::to_writer(&file, question_entry).unwrap();
                return;
            }
        }
    }

    #[tokio::test]
    pub async fn test_extract_parliamentary_answers_with_ai_content() {
        let pg = connect_pg().await;
        let res = extract_questions_with_ai_content(&pg).await.unwrap();

        for question_entry in &res {
            let answers = question_entry.answer.clone().unwrap_or_default();
            println!("answers: {answers:?}");
            if !answers.is_empty() {
                let file = std::fs::File::create("tests/question_response.json").unwrap();
                serde_json::to_writer(&file, question_entry).unwrap();
                return;
            }
        }
    }

    const TEST_JSON: &'static str = include_str!("../../../tests/question_response.json");

    #[test]
    fn test_question_answer_matching() {
        let question_entry_response: ParliamentInquiryResponse =
            serde_json::from_str(TEST_JSON).unwrap();
        // dbg!(&question_entry_response);
        let answer_entry = question_entry_response.answer.as_ref().unwrap()[0].clone();
        let question_entry = question_entry_response.question.unwrap();

        for question in &question_entry
            .ai_question
            .as_ref()
            .unwrap()
            .full_question_entry
            .questions
        {
            let answer_to_question = answer_entry
                .ai_answer
                .as_ref()
                .unwrap()
                .full_answer_entry
                .answers
                .iter()
                .find(|answer| {
                    answer
                        .answering_questions_references
                        .iter()
                        .find(|reference| {
                            reference.affected_question_absolute_path == question.nth_level
                        })
                        .is_some()
                });
            match answer_to_question {
                Some(atq) => {
                    // dbg!(&question.nth_level, &atq.answering_questions_references);
                    println!(
                        "question nthlevel: {:?}, answering question refs: {:?}",
                        question.nth_level, &atq.answering_questions_references
                    );
                }
                None => {
                    println!(
                        "question nth level does not not have corresponding answer: {:?}",
                        question.nth_level
                    );
                }
            }
            println!()
        }
        for answer in &answer_entry
            .ai_answer
            .as_ref()
            .unwrap()
            .full_answer_entry
            .answers
        {
            dbg!(&answer.raw_answer);
            for question_ref in &answer.answering_questions_references {
                dbg!(&question_ref.affected_question_absolute_path);
            }
        }
    }
}
