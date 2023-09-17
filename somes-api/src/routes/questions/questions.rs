use axum::{extract::Query, Json};
use dataservice::db::api_models::DbQuestionQuery;
use diesel::{sql_query, sql_types::Integer, PgConnection, QueryResult, RunQueryDsl};
use somes_common_lib::Page;

use crate::DataserviceDbConnection;

use super::error::QuestionErrorResponse;

pub async fn questions(
    Query(page): Query<Page>,
    DataserviceDbConnection(con): DataserviceDbConnection,
) -> Result<Json<Vec<DbQuestionQuery>>, QuestionErrorResponse> {
    con.interact(move |con| {
        get_questions(con, page.page)
            .map(Json)
            .map_err(|_| QuestionErrorResponse::DbInteraction)
    })
    .await
    .map_err(|_| QuestionErrorResponse::DbInteraction)?
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
    sql_query(
        "
        SELECT  *
        FROM    ( SELECT    ROW_NUMBER() OVER ( ORDER BY created_at DESC ) AS RowNum, *
                FROM      questions
                ) AS RowConstrainedResult
        WHERE   RowNum >= $1
            AND RowNum <= $2
        ORDER BY RowNum;
    ",
    )
    .bind::<Integer, _>(start)
    .bind::<Integer, _>(end)
    .load::<DbQuestionQuery>(con)
}

#[cfg(test)]
mod tests {
    use diesel::Connection;

    use crate::{dataservice::dataservice_con, routes::questions::ask_question::insert_question};

    use super::get_questions;

    #[test]
    fn test_get_questions() {
        let con = &mut dataservice_con();
        con.test_transaction::<_, (), _>(|con| {
            // may need to add a default user
            for i in 0..33 {
                insert_question(con, 1, format!("title: {i}"), format!("body: {i}"), 21029)
                    .unwrap();
            }
            let question_first_page = get_questions(con, 0).unwrap();
            assert_eq!(question_first_page.len(), 16);
            for (i, question) in question_first_page.iter().enumerate() {
                assert_eq!(question.title, format!("title: {i}"));
                assert_eq!(question.body, format!("body: {i}"));
                assert_eq!(question.issuer_id, 1);
                assert_eq!(question.delegate_id, 21029);
            }
            Ok(())
        });
    }
}
