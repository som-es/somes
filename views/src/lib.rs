use crate::{decrees::create_ministerial_decrees_with_docs_view, delegates::create_delegates_view};
use sqlx::{Executor, PgPool, Postgres, Transaction};

pub mod composite_types;
pub mod decrees;
pub mod delegates;

pub use composite_types::create_composite_types;

pub async fn create_views<'a>(tx: &mut Transaction<'a, Postgres>) -> sqlx::Result<()> {
    create_ministerial_decrees_with_docs_view(tx).await?;
    create_delegates_view(tx).await?;

    Ok(())
}
