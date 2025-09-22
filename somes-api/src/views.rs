use sqlx::PgPool;

use crate::routes::view::create_ministerial_decrees_with_docs_view;

pub async fn create_views(pool: &PgPool) -> sqlx::Result<()> {
    create_ministerial_decrees_with_docs_view(pool).await?;

    Ok(())
}
