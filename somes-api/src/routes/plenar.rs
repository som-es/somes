use std::collections::HashMap;

use axum::Json;
use combx::DbPlenar;
use sqlx::PgPool;

use crate::{GenericError, PgPoolConnection};

pub fn group_plenary_sessions_per_gp(
    plenary_sessions: Vec<DbPlenar>,
) -> HashMap<String, Vec<DbPlenar>> {
    let mut plenary_sessions_by_gp = HashMap::new();
    for plenary_session in plenary_sessions {
        if plenary_sessions_by_gp.contains_key(&plenary_session.legislative_period) {
            plenary_sessions_by_gp
                .entry(plenary_session.legislative_period.clone())
                .and_modify(|sessions: &mut Vec<DbPlenar>| sessions.push(plenary_session));
        } else {
            plenary_sessions_by_gp.insert(
                plenary_session.legislative_period.clone(),
                vec![plenary_session],
            );
        };
    }
    plenary_sessions_by_gp
}

pub async fn plenary_sessions_per_gp_route(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<HashMap<String, Vec<DbPlenar>>>, GenericError> {
    let plenary_sessions = extract_plenar_infos_sqlx(&pg)
        .await
        .map_err(|e| GenericError::SqlFailure(Some(e)))?;

    Ok(Json(group_plenary_sessions_per_gp(plenary_sessions)))
}

pub async fn extract_plenar_infos_sqlx(pg: &PgPool) -> sqlx::Result<Vec<DbPlenar>> {
    sqlx::query_as!(DbPlenar, "select * from plenar_infos order by inr asc")
        .fetch_all(pg)
        .await
}

#[cfg(test)]
mod tests {
    use combx::connect_pg;

    use crate::routes::{extract_plenar_infos_sqlx, group_plenary_sessions_per_gp};

    #[tokio::test]
    pub async fn test_extract_plenar_infos() {
        let pg = connect_pg().await;
        let res = extract_plenar_infos_sqlx(&pg).await.unwrap();

        let grouped = group_plenary_sessions_per_gp(res.clone());

        let mut sum = 0;
        for (_key, value) in grouped {
            sum += value.len();
        }

        assert_eq!(res.len(), sum);
        dbg!(res.len());
    }
}
