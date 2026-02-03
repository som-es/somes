use dataservice::combx::DbAiSummary;
use somes_common_lib::ToCompositeType;
use sqlx::{Postgres, Transaction};

pub async fn create_ministerial_decrees_with_docs_view<'a>(
    tx: &mut Transaction<'a, Postgres>,
) -> sqlx::Result<()> {

    let summary_fields = DbAiSummary::field_orders()
        .into_iter()
        .map(|field| if field == "id" { "s.id" } else { field })
        .collect::<Vec<_>>()
        .join(" ,");

    sqlx::query!("DROP VIEW IF EXISTS ministrial_decrees_with_docs;")
        .execute(&mut **tx)
        .await?;
    sqlx::query(
        &format!("
        CREATE VIEW ministrial_decrees_with_docs AS
        SELECT
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
            ARRAY(
                SELECT ROW(title, document_url, document_type)::document
                from ministrial_decrees_documents doc
                where doc.ministrial_decree_id = d.id
            ) as \"documents: Vec<Document>\",
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
    Ok(())
}
