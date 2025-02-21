use somes_common_lib::Absence;
use sqlx::{query_as, PgPool};

pub async fn extract_absences_by_delegate(pg: &PgPool, delegate_id: i32) -> sqlx::Result<Vec<Absence>> {
    query_as!(Absence, 
        "select add_date as date, plenary_session_id from absences inner join plenar_infos pi on pi.id = plenary_session_id where delegate_id = $1", 
        delegate_id
    ).fetch_all(pg).await
}
