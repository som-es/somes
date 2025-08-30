use dataservice::combx::{Document, Topic};
use sqlx::PgPool;

pub async fn sqlx_get_docs_from_ministerial_prop(
    con: &PgPool,
    ministrial_proposal_id: i32,
) -> sqlx::Result<Vec<Document>> {
    sqlx::query_as!(Document,
        "select title, document_url, document_type from ministrial_proposals_documents where ministrial_proposal_id = $1",
        ministrial_proposal_id
    )
    .fetch_all(con)
    .await
}

pub async fn sqlx_get_ministerial_issuers(
    con: &PgPool,
    ministrial_proposal_id: i32,
) -> sqlx::Result<Vec<i32>> {
    sqlx::query_scalar!(
        "select delegate_id from ministrial_issuer where ministrial_proposal_id = $1",
        ministrial_proposal_id
    )
    .fetch_all(con)
    .await
}

pub async fn sqlx_get_eurovoc_topics_from_ministrial_proposal(
    con: &PgPool,
    ministrial_proposal_id: i32,
) -> sqlx::Result<Vec<Topic>> {
    sqlx::query_as!(
        Topic,
        "select topic from eurovoc_topics_ministrial_proposals where ministrial_proposal_id = $1",
        ministrial_proposal_id
    )
    .fetch_all(con)
    .await
}

pub async fn sqlx_get_topics_from_ministrial_proposal(
    con: &PgPool,
    ministrial_proposal_id: i32,
) -> sqlx::Result<Vec<Topic>> {
    sqlx::query_as!(
        Topic,
        "select topic from topics_ministrial_proposals where ministrial_proposal_id = $1",
        ministrial_proposal_id
    )
    .fetch_all(con)
    .await
}

pub async fn sqlx_get_other_keyword_topics_from_ministrial_proposal(
    con: &PgPool,
    ministrial_proposal_id: i32,
) -> sqlx::Result<Vec<Topic>> {
    sqlx::query_as!(
        Topic,
        "select topic from other_keyword_topics_ministrial_proposals where ministrial_proposal_id = $1",
        ministrial_proposal_id
    )
    .fetch_all(con)
    .await
}
