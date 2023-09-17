use axum::{extract::Query, Json};
use dataservice::db::api_models::DbQuestionQuery;
use diesel::{PgConnection, sql_query, sql_types::Integer, RunQueryDsl, QueryResult};
use somes_common_lib::Page;

use crate::DataserviceDbConnection;

use super::error::QuestionErrorResponse;

pub async fn questions(
    Query(page): Query<Page>,
    DataserviceDbConnection(con): DataserviceDbConnection,
) -> Result<Json<Vec<DbQuestionQuery>>, QuestionErrorResponse> {
    con.interact(move |con| {
        get_questions(con, page.page).map(Json).map_err(|_| QuestionErrorResponse::DbInteraction)
    }).await.map_err(|_| QuestionErrorResponse::DbInteraction)?
}

/*

SELECT  *
FROM    ( SELECT    ROW_NUMBER() OVER ( ORDER BY created_at DESC ) AS RowNum, *
          FROM      question
        ) AS RowConstrainedResult
WHERE   RowNum >= 1
    AND RowNum < 20
ORDER BY RowNum;
*/

pub fn get_questions(con: &mut PgConnection, page: i32) -> QueryResult<Vec<DbQuestionQuery>> {
    let start = page * 16;
    let end = start + 16;
    sql_query("
        SELECT  *
        FROM    ( SELECT    ROW_NUMBER() OVER ( ORDER BY created_at DESC ) AS RowNum, *
                FROM      question
                ) AS RowConstrainedResult
        WHERE   RowNum >= $1
            AND RowNum < $2
        ORDER BY RowNum;
    ").bind::<Integer, _>(start)
    .bind::<Integer, _>(end).load::<DbQuestionQuery>(con)
}
