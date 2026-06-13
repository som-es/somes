use combx::{DbAnswerEntry, DbQuestionEntry};
use somes_common_lib::ToCompositeType;
use sqlx::{Postgres, Transaction};

pub async fn create_parliament_qa_view<'a>(tx: &mut Transaction<'a, Postgres>) -> sqlx::Result<()> {
    let question_fields = DbQuestionEntry::field_orders()
        .into_iter()
        .map(|field| if field == "id" { "q.id" } else { field })
        .collect::<Vec<_>>()
        .join(", ");

    let answer_fields = DbAnswerEntry::field_orders()
        .into_iter()
        .map(|field| if field == "id" { "a.id" } else { field })
        .collect::<Vec<_>>()
        .join(", ");

    sqlx::query!("DROP VIEW IF EXISTS pqa_composite_questions;")
        .execute(&mut **tx)
        .await?;

    sqlx::query(&format!(
        "
        CREATE VIEW pqa_composite_questions AS
        SELECT
            ROW(
                ROW(
                    m.gp,
                    m.ityp,
                    m.inr,
                    m.issuer_ids,
                    m.reciever_ids,
                    ARRAY(
                        SELECT ROW(doc.title, doc.document_url, doc.document_type)::document
                        FROM pqa_documents doc
                        WHERE doc.pqa_meta_id = m.id
                    ),
                    m.topics,
                    m.eurovoc_topics,
                    m.other_keyword_topics,
                    m.description,
                    m.title,
                    m.created_at,
                    m.updated_at,
                    m.raw_data_created_at,
                    m.raw_data_updated_at
                )::parliament_raw_data,
                (
                    SELECT ROW(
                        {question_fields}
                    )::db_question_entry
                    FROM pqa_questions q
                    WHERE q.pqa_meta_id = m.id
                    ORDER BY q.generated_at DESC
                    LIMIT 1
                ),
                ARRAY(
                    SELECT ROW(r.pqa_gp, r.pqa_ityp, r.pqa_inr)::db_reference
                    FROM pqa_references r
                    WHERE r.pqa_meta_id = m.id
                )
            )::parliament_question AS \"question: ParliamentQuestion\",
            ARRAY(
                SELECT ROW(
                    ROW(
                        m.gp,
                        m.ityp,
                        m.inr,
                        m.issuer_ids,
                        m.reciever_ids,
                        ARRAY(
                            SELECT ROW(doc.title, doc.document_url, doc.document_type)::document
                            FROM pqa_documents doc
                            WHERE doc.pqa_meta_id = m.id
                        ),
                        m.topics,
                        m.eurovoc_topics,
                        m.other_keyword_topics,
                        m.description,
                        m.title,
                        m.created_at,
                        m.updated_at,
                        m.raw_data_created_at,
                        m.raw_data_updated_at
                    )::parliament_raw_data,
                    ROW(
                        {answer_fields}
                    )::db_answer_entry,
                    ARRAY(
                        SELECT ROW(r.pqa_gp, r.pqa_ityp, r.pqa_inr)::db_reference
                        FROM pqa_references r
                        WHERE r.pqa_meta_id = m.id
                    )
                )::parliament_answer
                FROM pqa_answers a
                WHERE a.pqa_meta_id = m.id
                ORDER BY a.generated_at DESC
            ) AS \"answer: Vec<ParliamentAnswer>\"
        FROM pqa_meta m;
        "
    ))
    .execute(&mut **tx)
    .await?;

    Ok(())
}
