use dataservice::combx::{DelegateMatch, Interjection};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct InterjectionsWithMaxPage {
    pub interjections: Vec<Interjection>,
    pub entry_count: i64,
    pub max_page: i64,
}

pub async fn extract_interjections_made_by_delegate(
    delegate_id: i32,
    page: i64,
    page_elements: i64,
    pg_pool: &PgPool,
) -> sqlx::Result<InterjectionsWithMaxPage> {
    let all_interjections_count = sqlx::query_scalar!(
        r#"select COUNT(*) as "count!" from interjections where interjector_delegate_id = $1"#,
        delegate_id
    )
    .fetch_one(pg_pool)
    .await?;

    let interjections = sqlx::query_as!(
        Interjection,
        r#"
            SELECT
                i.interjection_text,
                i.interjector_delegate_id,
                i.plenar_speech_id,
                i.rel_start_idx,
                i.rel_end_idx,
                ROW(
                    dm.similiarity_score,
                    dm.searched_with,
                    dm.matched_with,
                    dm.delegate_id,
                    dm.manually_matched
                )::delegate_match as "delegate_match!: DelegateMatch"
            FROM interjections i
            INNER JOIN delegate_matching dm
                ON dm.id = i.delegate_matching_id
            WHERE i.interjector_delegate_id = $1 
            ORDER BY i.id
            OFFSET $2 LIMIT $3
        "#,
        delegate_id,
        page * page_elements,
        page_elements
    )
    .fetch_all(pg_pool)
    .await?;
    Ok(InterjectionsWithMaxPage {
        interjections,
        entry_count: page_elements,
        max_page: (all_interjections_count as f64 / page_elements as f64).ceil() as i64,
    })
}

pub async fn extract_interjections_received_by_delegate(
    delegate_id: i32,
    page: i64,
    page_elements: i64,
    pg_pool: &PgPool,
) -> sqlx::Result<InterjectionsWithMaxPage> {
    let all_interjections_count = sqlx::query_scalar!(
        r#"select COUNT(*) as "count!" from interjections i INNER JOIN delegate_matching dm
                ON dm.id = i.delegate_matching_id where dm.delegate_id = $1"#,
        delegate_id
    )
    .fetch_one(pg_pool)
    .await?;

    let interjections = sqlx::query_as!(
        Interjection,
        r#"
            SELECT
                i.interjection_text,
                i.interjector_delegate_id,
                i.plenar_speech_id,
                i.rel_start_idx,
                i.rel_end_idx,
                ROW(
                    dm.similiarity_score,
                    dm.searched_with,
                    dm.matched_with,
                    dm.delegate_id,
                    dm.manually_matched
                )::delegate_match as "delegate_match!: DelegateMatch"
            FROM interjections i
            INNER JOIN delegate_matching dm
                ON dm.id = i.delegate_matching_id
            WHERE dm.delegate_id = $1 
            ORDER BY i.id
            OFFSET $2 LIMIT $3
        "#,
        delegate_id,
        page * page_elements,
        page_elements
    )
    .fetch_all(pg_pool)
    .await?;

    Ok(InterjectionsWithMaxPage {
        interjections,
        entry_count: page_elements,
        max_page: (all_interjections_count as f64 / page_elements as f64).ceil() as i64,
    })
}
