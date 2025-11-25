use dataservice::combx::Decree;
use somes_common_lib::Document;
use sqlx::PgPool;

pub async fn extract_decrees_from_gov_official(
    delegate_id: i32,
    pg: &PgPool,
) -> sqlx::Result<Vec<Decree>> {
    Ok(sqlx::query!(
        r#"
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
            COALESCE(
                json_agg(
                    json_build_object(
                        'title', doc.title,
                        'document_type', doc.document_type,
                        'document_url', doc.document_url
                    )
                ),
                '[]'
            ) as documents
        FROM
            ministrial_decrees d
        LEFT JOIN
            ministrial_decrees_documents doc ON d.id = doc.ministrial_decree_id
        WHERE
            d.gov_official_id = $1
        GROUP BY
            d.gov_official_id, d.ris_id, d.ministrial_issuer,
            d.title, d.short_title, d.publication_date, d.part,
            d.emphasis, d.gp, d.eli, d.document_url
        "#,
        delegate_id
    )
    .fetch_all(pg)
    .await?
    .into_iter()
    .map(|x| Decree {
        gov_official_id: Some(x.gov_official_id),
        ris_id: x.ris_id,
        ministrial_issuer: x.ministrial_issuer,
        title: x.title,
        short_title: x.short_title,
        publication_date: x.publication_date,
        part: x.part,
        emphasis: x.emphasis,
        gp: x.gp,
        document_url: x.document_url.unwrap(),
        eli: x.eli.unwrap(),
        documents: x
            .documents
            .into_iter()
            .flat_map(|x| serde_json::from_value::<Document>(x))
            .collect(),
    })
    .collect())
}
