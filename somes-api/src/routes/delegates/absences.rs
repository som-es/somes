use somes_common_lib::Absence;
use sqlx::{query_as, PgPool};

pub async fn extract_absences_by_delegate(
    pg: &PgPool,
    delegate_id: i32,
) -> sqlx::Result<Vec<Absence>> {
    // mind that this only returns the accepted ones you know
    let absences = query_as!(Absence,
        "select raw_data_created_at as date, inr, legislative_period as gp, a.plenary_session_id, COALESCE(lis.li_ids, '{}') AS missed_legis_init_ids from absences a
        inner join 
            plenar_infos pi on pi.id = a.plenary_session_id 
        left join
            (select li.plenary_session_id, ARRAY_AGG(id) as li_ids from legislative_initiatives li where accepted is not null group by li.plenary_session_id) as lis
        on lis.plenary_session_id = a.plenary_session_id
        where delegate_id = $1
        order by raw_data_created_at desc, inr desc
        ", 

        delegate_id
    ).fetch_all(pg).await;

    absences
}
