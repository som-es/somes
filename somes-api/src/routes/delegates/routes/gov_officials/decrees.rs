use dataservice::combx::{DbAiSummary, Decree, OptionalDecree};
use somes_common_lib::Document;
use sqlx::PgPool;

pub async fn extract_decrees_from_gov_official(
    delegate_id: i32,
    pg: &PgPool,
) -> sqlx::Result<Vec<OptionalDecree>> {
    sqlx::query_as!(
        OptionalDecree,
        r#"

        select * from ministrial_decrees_with_docs
        where gov_official_id = $1
        order by publication_date desc
        
        "#,
        delegate_id
    )
    .fetch_all(pg)
    .await
}
