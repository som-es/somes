use dataservice::db::models::{DbDelegate, DbParty, DbProposalQuery};
use sqlx::PgPool;

pub async fn get_delegates(con: &PgPool) -> sqlx::Result<Vec<DbDelegate>> {
    sqlx::query_as!(
        DbDelegate,
        "
        SELECT * FROM delegates
        WHERE is_active = true
        AND council = 'nr'
        AND seat_row IS NOT NULL
        "
    )
    .fetch_all(con)
    .await
}

// pub fn get_delegates(con: &mut PgConnection) -> QueryResult<Vec<DbDelegate>> {
//     delegates
//         .filter(is_active.eq(true))
//         .filter(council.eq("nr"))
//         .filter(seat_row.is_not_null())
//         .load(con)
//     // delegates.load(con)
// }

pub async fn get_delegate(con: &PgPool, delegate_id: i32) -> sqlx::Result<DbDelegate> {
    sqlx::query_as!(
        DbDelegate,
        "
        SELECT * FROM delegates
        WHERE id = $1
        ",
        delegate_id
    )
    .fetch_one(con)
    .await
}

#[inline]
// pub fn get_delegate(con: &mut PgConnection, delegate_id: i32) -> QueryResult<DbDelegate> {
//     delegates
//         .filter(dataservice::db::schema::delegates::id.eq(delegate_id))
//         .first(con)
// }

pub async fn get_proposals(con: &PgPool) -> sqlx::Result<Vec<DbProposalQuery>> {
    sqlx::query_as!(
        DbProposalQuery,
        "
        SELECT * FROM proposals
        "
    )
    .fetch_all(con)
    .await
}

// pub fn get_proposals(con: &mut PgConnection) -> QueryResult<Vec<DbProposalQuery>> {
//     proposals.load::<DbProposalQuery>(con)
// }

// pub async fn get_speeches_from_legis_init(
//     con: &PgPool,
//     legis_init_id: i32,
// ) -> sqlx::Result<Vec<DbSpeech>> {
//     sqlx::query_as!(
//         DbSpeech,
//         "
//         SELECT * FROM speeches
//         WHERE legislative_initiatives_id = $1
//         ",
//         legis_init_id
//     )
//     .fetch_all(con)
//     .await
// }

// pub fn get_speeches_from_legis_init(
//     con: &mut PgConnection,
//     legis_init_id: i32,
// ) -> QueryResult<Vec<DbSpeech>> {
//     speeches
//         .filter(dataservice::db::schema::speeches::legislative_initiatives_id.eq(legis_init_id))
//         .load::<DbSpeech>(con)
// }

pub async fn get_parties(con: &PgPool) -> sqlx::Result<Vec<DbParty>> {
    sqlx::query_as!(
        DbParty,
        "
        SELECT name,fraction,color FROM parties
        "
    )
    .fetch_all(con)
    .await
}

// pub fn get_parties(con: &mut PgConnection) -> QueryResult<Vec<DbParty>> {
//     parties.load::<DbParty>(con)
// }
