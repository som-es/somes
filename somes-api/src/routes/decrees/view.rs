use sqlx::PgPool;

pub async fn create_ministerial_decrees_with_docs_view(pool: &PgPool) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
        CREATE OR REPLACE VIEW ministrial_decrees_with_docs AS
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
            COUNT(*) OVER() AS entry_count,
            COALESCE(
                json_agg(
                    json_build_object(
                        'title', doc.title,
                        'document_url', doc.document_url,
                        'document_type', doc.document_type
                    )
                ) FILTER (WHERE doc.id IS NOT NULL),
                '[]'
            ) as documents
        FROM
            ministrial_decrees d
        LEFT JOIN
            ministrial_decrees_documents doc ON d.id = doc.ministrial_decree_id
        GROUP BY
            d.gov_official_id, d.ris_id, d.ministrial_issuer,
            d.title, d.short_title, d.publication_date, d.part,
            d.emphasis, d.gp;
        "#,
    )
    .execute(pool)
    .await
    .map(|_x| ())
}
