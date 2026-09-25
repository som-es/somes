use combx::DbAiSummary;
use somes_common_lib::ToCompositeType;
use sqlx::{Postgres, Transaction};

pub async fn create_ministerial_decrees_with_docs_view<'a>(
    tx: &mut Transaction<'a, Postgres>,
    up: bool,
) -> sqlx::Result<()> {
    let summary_fields = DbAiSummary::field_orders()
        .into_iter()
        .map(|field| if field == "id" { "s.id" } else { field })
        .collect::<Vec<_>>()
        .join(" ,");

    sqlx::query!("DROP VIEW IF EXISTS ministrial_decrees_with_docs;")
        .execute(&mut **tx)
        .await?;
    if up {
        sqlx::query(&format!(
            "
        CREATE VIEW ministrial_decrees_with_docs AS
        SELECT
            d.id,
            d.gov_official_id,
            d.ris_id,
            d.ministrial_issuer,
            d.title,
            d.short_title,
            d.publication_date,
            d.part,
            d.emphasis,
            d.gp,
            d.eli,
            d.document_url,
            d.created_at,
            d.updated_at,
            d.is_norm,
            ARRAY(
                SELECT ROW(title, document_url, document_type)::document
                from ministrial_decrees_documents doc
                where doc.ministrial_decree_id = d.id
            ) as \"documents: Vec<Document>\",
            (
                SELECT inner_decrees.ris_id
                from ris_amendments ra
                inner join ministrial_decrees inner_decrees on inner_decrees.id = ra.norm_id
                where ra.decree_id = d.id
                limit 1
            ) as \"parent_decree_ris_id: String\",
            ARRAY(
                SELECT ROW(bgbl, ris_id)::db_ris_amendment_ref
                from ris_amendments
                where norm_id = d.id
            ) as \"amendments: Vec<DbRisAmendmentRef>\",
            (
            SELECT
              ROW(
                {summary_fields}
              )::db_ai_summary
            FROM
              decree_summaries mps
              inner join summaries s on s.id = mps.summary_id
            WHERE
              mps.decree_id = d.id
            order by
              s.generated_at DESC
            LIMIT 1
          ) AS \"ai_summary: DbAiSummary\"

        FROM ministrial_decrees d;
        "
        ))
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}
