use crate::routes::orientation_questions::{
    OrientationQuestionResponse, orientation_questions_route,
};
use sqlx::PgPool;

#[sqlx::test(fixtures("fixtures/orientation_questions_base.sql"))]
async fn test_orientation_questions_returns_expected_questions(pool: PgPool) {
    // Simulate the handler logic by calling the internal extraction directly
    // For simplicity we call the route handler via PgPoolConnection wrapper
    // The handler expects PgPoolConnection, we can construct it manually.
    use crate::PgPoolConnection;
    let pg_conn = PgPoolConnection(pool);
    // We need to call the handler function directly
    // The handler returns Result<Json<Vec<OrientationQuestionResponse>>, _>
    let json = orientation_questions_route(pg_conn).await.unwrap();
    let responses: Vec<OrientationQuestionResponse> = json.0;

    // We expect 2 questions where is_part_of IS NOT NULL
    assert_eq!(responses.len(), 2);
    assert_eq!(responses[0].id, 1);
    assert_eq!(responses[1].id, 2);

    // Question 1 checks
    let q1 = &responses[0];
    assert_eq!(q1.question, "Should taxes be raised?");
    assert_eq!(q1.is_left, Some(true));
    assert_eq!(q1.is_liberal, Some(false));
    assert_eq!(
        q1.is_part_of,
        vec!["economy".to_string(), "tax".to_string()]
    );
    assert_eq!(q1.strong_reference_answers.len(), 2);
    assert_eq!(
        q1.topics,
        vec!["Economy".to_string(), "Taxation".to_string()]
    );
    assert_eq!(q1.topics_influence.len(), 2);
    assert_eq!(
        q1.detailed_topics,
        vec!["Income Tax".to_string(), "Corporate Tax".to_string()]
    );
    assert_eq!(q1.detailed_topics_influence.len(), 2);

    // Question 2 checks
    let q2 = &responses[1];
    assert_eq!(q2.question, "Is climate action needed?");
    assert_eq!(q2.is_left, Some(false));
    assert_eq!(q2.is_liberal, Some(true));
    assert_eq!(q2.is_part_of, vec!["environment".to_string()]);
    assert_eq!(q2.strong_reference_answers.len(), 1);
    assert_eq!(q2.topics, vec!["Environment".to_string()]);
}
