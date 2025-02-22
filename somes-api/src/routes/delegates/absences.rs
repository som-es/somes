use somes_common_lib::Absence;
use sqlx::{query_as, PgPool};

pub async fn extract_absences_by_delegate(pg: &PgPool, delegate_id: i32) -> sqlx::Result<Vec<Absence>> {
    query_as!(Absence, 
        "select add_date as date, a.plenary_session_id, COALESCE(lis.li_ids, '{}') AS missed_legis_init_ids from absences a
        inner join 
            plenar_infos pi on pi.id = a.plenary_session_id 
        left join
            (select li.plenary_session_id, ARRAY_AGG(id) as li_ids from legislative_initiatives li group by li.plenary_session_id) as lis
        on lis.plenary_session_id = a.id
        where delegate_id = $1", 

        delegate_id
    ).fetch_all(pg).await
}
