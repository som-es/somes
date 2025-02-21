use somes_common_lib::NamedVote;
use sqlx::{query_as, PgPool};

pub async fn extract_named_votes_by_delegate(pg: &PgPool, delegate_id: i32) -> sqlx::Result<Vec<NamedVote>> {
    query_as!(NamedVote, 
        "select 
            infavor, was_absent, legis_init_id, named_vote_info_id, created_at as date
        from 
            named_votes 
        inner join named_vote_info nvi on nvi.id = named_vote_info_id 
        inner join legislative_initiatives li on li.id = legis_init_id 
        where delegate_id = $1
        order by date desc
        ", 
        delegate_id
    ).fetch_all(pg).await
}

