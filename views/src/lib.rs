use sqlx::PgPool;

use crate::decrees::create_ministerial_decrees_with_docs_view;

pub mod decrees;

pub async fn create_views(pool: &PgPool) -> sqlx::Result<()> {
    create_ministerial_decrees_with_docs_view(pool).await?;

    Ok(())
}
