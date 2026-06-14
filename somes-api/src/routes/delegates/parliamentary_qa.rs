use combx::models::*;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Deserialize, Serialize)]
pub struct ParliamentQuestionResponseWithMaxPage {
    pub question_entries: Vec<ParliamentQuestionResponse>,
    pub entry_count: i64,
    pub max_page: i64,
}

impl ParliamentQuestionResponseWithMaxPage {
    fn new(
        question_entries: Vec<ParliamentQuestionResponse>,
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
) -> sqlx::Result<ParliamentQuestionResponseWithMaxPage> {
    let offset = (page - 1) * page_elements;

    let rows = sqlx::query_as!(
        ParliamentQuestionResponse,
        r#"
        SELECT *
        FROM pqa_composite_questions
        WHERE $1 = ANY((("question: ParliamentQuestion").data).issuer_ids)
        ORDER BY (("question: ParliamentQuestion").data).created_at DESC
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
        WHERE $1 = ANY((("question: ParliamentQuestion").data).issuer_ids)
        "#,
        delegate_id as i64,
    )
    .fetch_one(pg_pool)
    .await?
    .unwrap_or(0);

    Ok(ParliamentQuestionResponseWithMaxPage::new(
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
) -> sqlx::Result<ParliamentQuestionResponseWithMaxPage> {
    let offset = (page - 1) * page_elements;

    let rows = sqlx::query_as!(
        ParliamentQuestionResponse,
        r#"
        SELECT *
        FROM pqa_composite_questions
        WHERE $1 = ANY((("question: ParliamentQuestion").data).receiver_ids)
        ORDER BY (("question: ParliamentQuestion").data).created_at DESC
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
        WHERE $1 = ANY((("question: ParliamentQuestion").data).receiver_ids)
        "#,
        delegate_id as i64,
    )
    .fetch_one(pg_pool)
    .await?
    .unwrap_or(0);

    Ok(ParliamentQuestionResponseWithMaxPage::new(
        rows,
        page_elements,
        all_entries_count,
    ))
}

#[cfg(test)]
mod tests {
    use crate::routes::extract_parliamentary_answers;
    use combx::connect_pg;

    #[tokio::test]
    pub async fn test_extract_parliamentary_answers_for_gov_official() {
        let pg = connect_pg().await;
        let res = extract_parliamentary_answers(20445, 1, 20, &pg)
            .await
            .unwrap();
        println!("total: {}, max page: {}", res.entry_count, res.max_page);
        println!("returned: {}", res.question_entries.len());
    }
}
