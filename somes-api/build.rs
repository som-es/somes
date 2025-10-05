use dotenvy_macro::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub const DATASERVICE_URL: &str = dotenv!("DATASERVICE_URL");

pub async fn create_ministerial_decrees_with_docs_view(pool: &PgPool) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query!("DROP VIEW IF EXISTS ministrial_decrees_with_docs;")
        .execute(&mut *tx)
        .await?;
    sqlx::query!(
        r#"
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
        FROM ministrial_decrees d
        LEFT JOIN ministrial_decrees_documents doc
            ON d.id = doc.ministrial_decree_id
        GROUP BY
            d.gov_official_id, d.ris_id, d.ministrial_issuer,
            d.title, d.short_title, d.publication_date, d.part,
            d.emphasis, d.gp, d.eli, d.document_url;
        "#
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await
}

#[tokio::main]
async fn main() {
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=build.rs");
    let dataservice_sqlx_pool = PgPoolOptions::new()
        // pool sizes
        .max_connections(1)
        .connect(DATASERVICE_URL)
        .await
        .unwrap();

    create_ministerial_decrees_with_docs_view(&dataservice_sqlx_pool)
        .await
        .unwrap();
}
