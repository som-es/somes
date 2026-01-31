use std::collections::HashMap;

use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use utoipa::ToSchema;

use crate::{GenericError, PgPoolConnection};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct Department {
    pub ressort: String,
    pub gp: String,
}

pub async fn departments(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<Vec<Department>>, GenericError> {
    query_as!(
        Department,
        "select gp, ressort from ministrial_proposals group by ressort, gp;"
    )
    .fetch_all(&pg)
    .await
    .map(Json)
    .map_err(|e| GenericError::SqlFailure(Some(e)))
}

pub async fn departments_per_gp(
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<Json<HashMap<String, Vec<String>>>, GenericError> {
    let departments = departments(PgPoolConnection(pg)).await?;

    let mut departments_per_gp = HashMap::<String, Vec<String>>::new();

    for department in departments.0 {
        departments_per_gp
            .entry(department.gp)
            .and_modify(|depts| depts.push(department.ressort.clone()))
            .or_insert(vec![department.ressort.clone()]);
    }

    Ok(Json(departments_per_gp))
}
