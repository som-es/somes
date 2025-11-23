use somes_common_lib::{FullMandate, ToCompositeType};
use sqlx::PgPool;

#[macro_export]
macro_rules! run_composite_type_creation {
    ($pool:ident, $($ty:ident),*) => {
        $(
            create_composite_type::<$ty>($pool).await?;
        )*
    };
}

pub async fn create_composite_types(pool: &PgPool) -> sqlx::Result<()> {
    run_composite_type_creation!(pool, FullMandate);
    Ok(())
}

pub async fn create_composite_type<T: ToCompositeType>(pg: &sqlx::PgPool) -> sqlx::Result<()> {
    let mut tx = pg.begin().await?;
    let create_composite_type_str = T::to_sql_create_composite_type();
    sqlx::query(&format!("DROP TYPE IF EXISTS {} cascade", T::type_name()))
        .execute(&mut *tx)
        .await?;
    sqlx::query(&create_composite_type_str)
        .execute(&mut *tx)
        .await?;
    tx.commit().await
}
