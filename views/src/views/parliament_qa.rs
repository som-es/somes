use combx::{DbAiInquiry, DbAnswerEntry};
use somes_common_lib::ToCompositeType;
use sqlx::{Postgres, Transaction};

pub async fn create_parliament_qa_view<'a>(
    tx: &mut Transaction<'a, Postgres>,
    up: bool,
) -> sqlx::Result<()> {
    let question_fields = DbAiInquiry::field_orders()
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

    if up {
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
                    )::db_ai_inquiry
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
            )::parliament_inquiry AS \"question: ParliamentInquiry\",
            ARRAY(
                SELECT ROW(
                    ROW(
                        am.gp,
                        am.ityp,
                        am.inr,
                        am.issuer_ids,
                        am.reciever_ids,
                        ARRAY(
                            SELECT ROW(doc.title, doc.document_url, doc.document_type)::document
                            FROM pqa_documents doc
                            WHERE doc.pqa_meta_id = am.id
                        ),
                        am.topics,
                        am.eurovoc_topics,
                        am.other_keyword_topics,
                        am.description,
                        am.title,
                        am.created_at,
                        am.updated_at,
                        am.raw_data_created_at,
                        am.raw_data_updated_at
                    )::parliament_raw_data,
                    (
                        select ROW(
                            {answer_fields}
                        )::db_answer_entry
                        FROM pqa_answers a
                        WHERE a.pqa_meta_id = am.id
                        ORDER BY a.generated_at DESC
                        LIMIT 1
                    ),
                    ARRAY(
                        SELECT ROW(r.pqa_gp, r.pqa_ityp, r.pqa_inr)::db_reference
                        FROM pqa_references r
                        WHERE r.pqa_meta_id = am.id
                    )
                )::parliament_answer

                FROM pqa_references r
                inner join pqa_meta am on am.gp = r.pqa_gp and am.inr = r.pqa_inr and am.ityp = r.pqa_ityp
                WHERE r.pqa_meta_id = m.id and r.pqa_ityp like 'AB%'
            ) AS \"answer: Vec<ParliamentAnswer>\"
        FROM pqa_meta m
        where m.ityp like 'J%'
        "
    ))
    .execute(&mut **tx)
    .await?;
    }

    Ok(())
}
