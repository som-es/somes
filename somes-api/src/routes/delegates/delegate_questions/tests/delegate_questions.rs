use common_scrapes::language::Language;
use reqwest::StatusCode;
use sqlx::PgPool;

use crate::GenericError;

use super::{
    db::{
        create_question, fetch_public_questions, fetch_question_topics, fetch_review_questions,
        find_admin_question, find_public_question, update_question,
    },
    models::{DelegateQuestionTopic, QuestionDelivery},
};

fn expected_topics(topic: &[(&str, &str)]) -> Vec<DelegateQuestionTopic> {
    topic
        .iter()
        .map(|(id, name)| DelegateQuestionTopic {
            id: (*id).to_string(),
            topic: (*name).to_string(),
        })
        .collect()
}

fn topic_ids(ids: &[i64]) -> Vec<String> {
    ids.iter().map(|id| id.to_string()).collect()
}

#[sqlx::test(fixtures("fixtures/delegate_questions_base.sql"))]
async fn public_questions_expose_topics_in_requested_language(pool: PgPool) {
    let questions = fetch_public_questions(&pool, None, Language::En)
        .await
        .unwrap();

    assert_eq!(questions.len(), 2);

    assert_eq!(questions[0].subject, "Frage zwei");
    assert_eq!(questions[0].delegate_id, 1);
    assert_eq!(
        questions[0].topics,
        expected_topics(&[("200", "professional association")])
    );
    assert_eq!(questions[0].answers.len(), 2);
    assert_eq!(questions[0].answers[0].body, "erste Antwort");
    assert_eq!(questions[0].answers[1].body, "zweite Antwort");

    assert_eq!(questions[1].subject, "Frage eins");
    assert_eq!(
        questions[1].topics,
        expected_topics(&[("100", "committee report"), ("300", "Nur Deutsch")])
    );
    assert!(questions[1].answers.is_empty());
}

#[sqlx::test(fixtures("fixtures/delegate_questions_base.sql"))]
async fn find_public_question_returns_topics_and_answers(pool: PgPool) {
    let english = find_public_question(&pool, 2, Language::En).await.unwrap();

    assert_eq!(english.id, 2);
    assert_eq!(english.subject, "Frage zwei");
    assert_eq!(english.delegate_id, 1);
    assert_eq!(
        english.topics,
        expected_topics(&[("200", "professional association")])
    );
    assert_eq!(english.answers.len(), 2);

    let german = find_public_question(&pool, 1, Language::De).await.unwrap();

    assert_eq!(german.id, 1);
    assert_eq!(german.subject, "Frage eins");
    assert_eq!(
        german.topics,
        expected_topics(&[("100", "Ausschussbericht"), ("300", "Nur Deutsch")])
    );
    assert!(german.answers.is_empty());
}

#[sqlx::test(fixtures("fixtures/delegate_questions_base.sql"))]
async fn find_public_question_rejects_unpublished_or_unknown_questions(pool: PgPool) {
    for question_id in [3, 4, 999] {
        let error = find_public_question(&pool, question_id, Language::De)
            .await
            .expect_err("only published questions may be fetched");

        let GenericError::Custom((status, reason)) = error else {
            panic!("expected a custom error, got {error:?}");
        };
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(reason, "Question was not found");
    }
}

#[sqlx::test(fixtures("fixtures/delegate_questions_base.sql"))]
async fn public_questions_are_filtered_by_delegate(pool: PgPool) {
    let anna = fetch_public_questions(&pool, Some(1), Language::De)
        .await
        .unwrap();
    assert_eq!(anna.len(), 2);

    let no_published_questions = fetch_public_questions(&pool, Some(2), Language::De)
        .await
        .unwrap();
    assert!(no_published_questions.is_empty());
}

#[sqlx::test(fixtures("fixtures/delegate_questions_base.sql"))]
async fn topics_fall_back_to_german_for_untranslated_topics(pool: PgPool) {
    let english = fetch_question_topics(&pool, &[1, 2], Language::En)
        .await
        .unwrap();

    assert_eq!(
        english[&1],
        expected_topics(&[("100", "committee report"), ("300", "Nur Deutsch")])
    );
    assert_eq!(
        english[&2],
        expected_topics(&[("200", "professional association")])
    );

    let german = fetch_question_topics(&pool, &[1], Language::De)
        .await
        .unwrap();
    assert_eq!(
        german[&1],
        expected_topics(&[("100", "Ausschussbericht"), ("300", "Nur Deutsch")])
    );

    let unknown_questions = fetch_question_topics(&pool, &[999], Language::De)
        .await
        .unwrap();
    assert!(unknown_questions.is_empty());

    let no_ids = fetch_question_topics(&pool, &[], Language::De)
        .await
        .unwrap();
    assert!(no_ids.is_empty());
}

#[sqlx::test(fixtures("fixtures/delegate_questions_base.sql"))]
async fn review_questions_include_topics(pool: PgPool) {
    let questions = fetch_review_questions(&pool, Language::De).await.unwrap();

    let ids: Vec<i64> = questions.iter().map(|question| question.id).collect();
    assert_eq!(ids, vec![3, 5]);

    assert_eq!(questions[0].status, "pending");
    assert_eq!(questions[0].delegate_name, "Berta Beta");
    assert_eq!(
        questions[0].topics,
        expected_topics(&[("100", "Ausschussbericht")])
    );

    assert_eq!(questions[1].status, "failed");
    assert_eq!(
        questions[1].topics,
        expected_topics(&[("300", "Nur Deutsch")])
    );
}

#[sqlx::test(fixtures("fixtures/delegate_questions_base.sql"))]
async fn find_admin_question_returns_topics_in_requested_language(pool: PgPool) {
    let question = find_admin_question(&pool, 1, Language::En).await.unwrap();

    assert_eq!(question.id, 1);
    assert_eq!(question.delegate_name, "Anna Alpha");
    assert_eq!(question.status, "sent");
    assert_eq!(
        question.topics,
        expected_topics(&[("100", "committee report"), ("300", "Nur Deutsch")])
    );
}

#[sqlx::test(fixtures("fixtures/delegate_questions_base.sql"))]
async fn update_question_rephrases_and_replaces_topics(pool: PgPool) {
    update_question(
        &pool,
        1,
        Some("neuer Titel"),
        Some("neuer Text"),
        Some(&["200".to_string(), "200".to_string()]),
    )
    .await
    .unwrap();

    let question = find_admin_question(&pool, 1, Language::De).await.unwrap();

    assert_eq!(question.subject, "neuer Titel");
    assert_eq!(question.body, "neuer Text");
    assert_eq!(
        question.topics,
        expected_topics(&[("200", "Berufsverband")])
    );
}

#[sqlx::test(fixtures("fixtures/delegate_questions_base.sql"))]
async fn update_question_can_rewrite_subject_without_touching_topics(pool: PgPool) {
    update_question(&pool, 1, Some("nur der Titel"), None, None)
        .await
        .unwrap();

    let question = find_admin_question(&pool, 1, Language::De).await.unwrap();

    assert_eq!(question.subject, "nur der Titel");
    assert_eq!(question.body, "Text eins");
    assert_eq!(
        question.topics,
        expected_topics(&[("100", "Ausschussbericht"), ("300", "Nur Deutsch")])
    );
}

#[sqlx::test(fixtures("fixtures/delegate_questions_base.sql"))]
async fn update_question_rolls_back_unknown_topic(pool: PgPool) {
    let error = update_question(
        &pool,
        1,
        Some("neuer Titel"),
        None,
        Some(&["404".to_string()]),
    )
    .await
    .expect_err("an unknown topic id must be rejected");

    let GenericError::Custom((status, reason)) = error else {
        panic!("expected a custom error, got {error:?}");
    };
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(reason, "unknown topic id");

    let question = find_admin_question(&pool, 1, Language::De).await.unwrap();

    assert_eq!(question.subject, "Frage eins");
    assert_eq!(
        question.topics,
        expected_topics(&[("100", "Ausschussbericht"), ("300", "Nur Deutsch")])
    );
}

#[sqlx::test(fixtures("fixtures/delegate_questions_base.sql"))]
async fn create_question_links_topics(pool: PgPool) {
    let question_id = create_question(
        &pool,
        1,
        1,
        "anna@example.at",
        QuestionDelivery::Delegate,
        "Anna Alpha",
        "neue Frage",
        "neuer Text",
        "msg-new@local",
        &topic_ids(&[100, 200]),
    )
    .await
    .unwrap();

    let question = find_admin_question(&pool, question_id, Language::En)
        .await
        .unwrap();

    assert_eq!(question.status, "pending");
    assert_eq!(question.delegate_name, "Anna Alpha");
    assert_eq!(
        question.topics,
        expected_topics(&[
            ("100", "committee report"),
            ("200", "professional association")
        ])
    );
}

#[sqlx::test(fixtures("fixtures/delegate_questions_base.sql"))]
async fn create_question_rejects_unknown_topic(pool: PgPool) {
    let error = create_question(
        &pool,
        1,
        1,
        "anna@example.at",
        QuestionDelivery::Delegate,
        "Anna Alpha",
        "neue Frage",
        "neuer Text",
        "msg-invalid@local",
        &topic_ids(&[404]),
    )
    .await
    .expect_err("an unknown topic id must be rejected");

    let GenericError::Custom((status, reason)) = error else {
        panic!("expected a custom error, got {error:?}");
    };
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(reason, "unknown topic id");

    let review = fetch_review_questions(&pool, Language::De).await.unwrap();
    assert_eq!(review.len(), 2);
}
