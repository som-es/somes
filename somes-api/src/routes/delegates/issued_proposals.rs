use somes_common_lib::IssuedProposal;
use sqlx::PgPool;

pub(crate) async fn extract_issued_proposals_by_delegate(
    delegate_id: i32,
    pg_pool: &PgPool,
) -> sqlx::Result<Vec<IssuedProposal>> {
    sqlx::query_as!(
        IssuedProposal,
        "
        select legis_init_id 
            from 
        legis_init_delegates lid 
        inner join legislative_initiatives li on li.id = lid.legis_init_id 
        where is_voteable_on and delegate_id = $1
    ",
        delegate_id
    )
    .fetch_all(pg_pool)
    .await
}
