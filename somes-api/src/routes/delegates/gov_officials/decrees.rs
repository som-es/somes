use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool, Postgres};
use utoipa::ToSchema;

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Document {
    pub title: String,
    pub link: String,
    pub ftype: String,
}

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Decree {
    gov_official_id: i32,
    ris_id: String,
    ministrial_issuer: String,
    title: String,
    short_title: String,
    publication_date: NaiveDate,
    part: String,
    documents: Vec<Document>,
}

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
            COALESCE(
                json_agg(
                    json_build_object(
                        'title', doc.title,
                        'ftype', doc.document_type,
                        'link', doc.document_url
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
            d.title, d.short_title, d.publication_date, d.part
        "#,
        delegate_id
    )
    .fetch_all(pg)
    .await?
    .into_iter()
    .map(|x| Decree {
        gov_official_id: x.gov_official_id,
        ris_id: x.ris_id,
        ministrial_issuer: x.ministrial_issuer,
        title: x.title,
        short_title: x.short_title,
        publication_date: x.publication_date,
        part: x.part,
        documents: x
            .documents
            .into_iter()
            .flat_map(|x| serde_json::from_value::<Document>(x))
            .collect(),
    }).collect())
}
