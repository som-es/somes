use super::*;

fn create_test_base_data() -> Vec<ActivityBase> {
    vec![
        ActivityBase {
            delegate_name: "Delegate A".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: "M".to_string(),
            activity_score: 2.5,
            raw_activity_score: 5.0,
            total_proposals: 10,
            session_count: 2,
            legislative_period: Some("XXV".to_string()),
            delegate_age_bucket: "41-50".to_string(),
        },
        ActivityBase {
            delegate_name: "Delegate B".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: "F".to_string(),
            activity_score: 1.5,
            raw_activity_score: 3.0,
            total_proposals: 5,
            session_count: 2,
            legislative_period: Some("XXV".to_string()),
            delegate_age_bucket: "31-40".to_string(),
        },
        ActivityBase {
            delegate_name: "Delegate C".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: "M".to_string(),
            activity_score: 3.0,
            raw_activity_score: 6.0,
            total_proposals: 8,
            session_count: 2,
            legislative_period: Some("XXV".to_string()),
            delegate_age_bucket: "51-60".to_string(),
        },
        ActivityBase {
            delegate_name: "Delegate D".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: "F".to_string(),
            activity_score: 2.0,
            raw_activity_score: 4.0,
            total_proposals: 12,
            session_count: 2,
            legislative_period: Some("XXVII".to_string()),
            delegate_age_bucket: "41-50".to_string(),
        },
    ]
}

#[test]
fn test_aggregate_by_party() {
    let base_data = create_test_base_data();
    let results = ActivityService::aggregate_by_party(base_data, true, true);

    // Verify Party X: avg_norm = (2.5 + 1.5) / 2 = 2.0, avg_raw = (5.0 + 3.0) / 2 = 4.0
    let party_x = results.iter().find(|r| r.category == "Party X").unwrap();
    assert!((party_x.activity_score - 2.0).abs() < 0.001);
    assert!((party_x.raw_activity_score - 4.0).abs() < 0.001);
    assert_eq!(party_x.total_proposals, 15);
    assert_eq!(party_x.delegate_count, 2);

    // Verify Party Y: avg_norm = (3.0 + 2.0) / 2 = 2.5, avg_raw = (6.0 + 4.0) / 2 = 5.0
    let party_y = results.iter().find(|r| r.category == "Party Y").unwrap();
    assert!((party_y.activity_score - 2.5).abs() < 0.001);
    assert!((party_y.raw_activity_score - 5.0).abs() < 0.001);
    assert_eq!(party_y.total_proposals, 20);
    assert_eq!(party_y.delegate_count, 2);
}

#[test]
fn test_aggregate_by_party_desc_false() {
    let base_data = create_test_base_data();
    let results = ActivityService::aggregate_by_party(base_data, false, true);

    assert!(results[0].activity_score < results[1].activity_score);
}

#[test]
fn test_aggregate_by_gender() {
    let base_data = create_test_base_data();
    let results = ActivityService::aggregate_by_gender(base_data, true, true);

    // Verify M: avg_norm = (2.5 + 3.0) / 2 = 2.75, avg_raw = (5.0 + 6.0) / 2 = 5.5
    let male = results.iter().find(|r| r.category == "M").unwrap();
    assert!((male.activity_score - 2.75).abs() < 0.001);
    assert!((male.raw_activity_score - 5.5).abs() < 0.001);
    assert_eq!(male.total_proposals, 18);
    assert_eq!(male.delegate_count, 2);

    // Verify F: avg_norm = (1.5 + 2.0) / 2 = 1.75, avg_raw = (3.0 + 4.0) / 2 = 3.5
    let female = results.iter().find(|r| r.category == "F").unwrap();
    assert!((female.activity_score - 1.75).abs() < 0.001);
    assert!((female.raw_activity_score - 3.5).abs() < 0.001);
    assert_eq!(female.total_proposals, 17);
    assert_eq!(female.delegate_count, 2);
}

#[test]
fn test_aggregate_by_legis() {
    let base_data = create_test_base_data();
    let results = ActivityService::aggregate_by_legis(base_data, true, true);

    // Verify 51: avg_norm = (2.5 + 1.5 + 3.0) / 3 = 2.333..., avg_raw = (5.0 + 3.0 + 6.0) / 3 = 4.666...
    let period_51 = results.iter().find(|r| r.category == "XXV").unwrap();
    assert!((period_51.activity_score - 2.3333333).abs() < 0.001);
    assert!((period_51.raw_activity_score - 4.6666667).abs() < 0.001);
    assert_eq!(period_51.total_proposals, 23);
    assert_eq!(period_51.delegate_count, 3);

    // Verify 52: avg_norm = 2.0, avg_raw = 4.0
    let period_52 = results.iter().find(|r| r.category == "XXVII").unwrap();
    assert!((period_52.activity_score - 2.0).abs() < 0.001);
    assert!((period_52.raw_activity_score - 4.0).abs() < 0.001);
    assert_eq!(period_52.total_proposals, 12);
    assert_eq!(period_52.delegate_count, 1);
}

#[test]
fn test_aggregate_by_age() {
    let base_data = create_test_base_data();
    let results = ActivityService::aggregate_by_age(base_data, true, true);

    // Verify 41-50: avg_norm = (2.5 + 2.0) / 2 = 2.25, avg_raw = (5.0 + 4.0) / 2 = 4.5
    let age_41_50 = results.iter().find(|r| r.category == "41-50").unwrap();
    assert!((age_41_50.activity_score - 2.25).abs() < 0.001);
    assert!((age_41_50.raw_activity_score - 4.5).abs() < 0.001);
    assert_eq!(age_41_50.total_proposals, 22);
    assert_eq!(age_41_50.delegate_count, 2);

    // Verify 31-40: avg_norm = 1.5, avg_raw = 3.0
    let age_31_40 = results.iter().find(|r| r.category == "31-40").unwrap();
    assert!((age_31_40.activity_score - 1.5).abs() < 0.001);
    assert!((age_31_40.raw_activity_score - 3.0).abs() < 0.001);
    assert_eq!(age_31_40.total_proposals, 5);
    assert_eq!(age_31_40.delegate_count, 1);

    // Verify 51-60: avg_norm = 3.0, avg_raw = 6.0
    let age_51_60 = results.iter().find(|r| r.category == "51-60").unwrap();
    assert!((age_51_60.activity_score - 3.0).abs() < 0.001);
    assert!((age_51_60.raw_activity_score - 6.0).abs() < 0.001);
    assert_eq!(age_51_60.total_proposals, 8);
    assert_eq!(age_51_60.delegate_count, 1);
}

#[test]
fn test_aggregate_by_party_sorts_by_raw_score() {
    let base_data = vec![
        ActivityBase {
            delegate_name: "Delegate A".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: "M".to_string(),
            activity_score: 10.0,
            raw_activity_score: 1.0,
            total_proposals: 1,
            session_count: 1,
            legislative_period: Some("XXV".to_string()),
            delegate_age_bucket: "41-50".to_string(),
        },
        ActivityBase {
            delegate_name: "Delegate B".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: "F".to_string(),
            activity_score: 1.0,
            raw_activity_score: 10.0,
            total_proposals: 1,
            session_count: 1,
            legislative_period: Some("XXV".to_string()),
            delegate_age_bucket: "31-40".to_string(),
        },
    ];

    let results = ActivityService::aggregate_by_party(base_data, true, false);

    assert_eq!(results[0].category, "Party Y");
}

#[tokio::test]
async fn test_get_base_data_applies_filters_and_computes_activity_stats() {
    let test_db = super::super::test_db::statistics_test_db(
        "test_get_base_data_applies_filters_and_computes_activity_stats",
    )
    .await;
    let pool = test_db.pool().clone();
    let filter = ActivityFilter {
        legis_period: Some("XXV".to_string()),
        party: Some("Party X".to_string()),
        gender: Some("M".to_string()),
        ..Default::default()
    };

    let results = ActivityService::get_base_data(&pool, &filter)
        .await
        .unwrap();

    assert_eq!(results.len(), 1);

    let delegate = &results[0];
    assert_eq!(delegate.delegate_name, "Delegate A");
    assert_eq!(delegate.delegate_party, "Party X");
    assert_eq!(delegate.delegate_gender, "M");
    assert_eq!(delegate.total_proposals, 2);
    assert_eq!(delegate.session_count, 1);
    assert!((delegate.raw_activity_score - 2.25).abs() < 0.001);
    assert!((delegate.activity_score - 2.25).abs() < 0.001);
    assert_eq!(delegate.legislative_period, Some("XXV".to_string()));
    assert_eq!(delegate.delegate_age_bucket, "31-40");
}

#[tokio::test]
async fn test_get_base_data_returns_empty_for_filter_without_matches() {
    let test_db = super::super::test_db::statistics_test_db(
        "test_get_base_data_returns_empty_for_filter_without_matches",
    )
    .await;
    let pool = test_db.pool().clone();
    let filter = ActivityFilter {
        legis_period: Some("XXV".to_string()),
        party: Some("Does Not Exist".to_string()),
        ..Default::default()
    };

    let results = ActivityService::get_base_data(&pool, &filter)
        .await
        .unwrap();

    assert!(results.is_empty());
}

#[tokio::test]
async fn test_per_delegate_aggregates_all_periods_into_one_delegate_row() {
    let test_db = super::super::test_db::statistics_test_db(
        "test_per_delegate_aggregates_all_periods_into_one_delegate_row",
    )
    .await;
    let pool = test_db.pool().clone();
    let filter = ActivityFilter {
        is_desc: true,
        normalized: true,
        ..Default::default()
    };

    let results = ActivityService::per_delegate(&pool, &filter).await.unwrap();

    assert_eq!(
        results
            .iter()
            .filter(|r| r.delegate_name == "Delegate D")
            .count(),
        1
    );

    let delegate = results
        .iter()
        .find(|r| r.delegate_name == "Delegate D")
        .unwrap();
    assert_eq!(delegate.delegate_party, "Party Y");
    assert!((delegate.raw_activity_score - 1.35).abs() < 0.001);
    assert!((delegate.activity_score - 0.675).abs() < 0.001);
    assert_eq!(delegate.total_proposals, 2);
    assert_eq!(delegate.session_count, 2);
}

#[tokio::test]
async fn test_legislative_initiatives_without_simple_majority_applies_filters() {
    let test_db = super::super::test_db::statistics_test_db(
        "test_legislative_initiatives_without_simple_majority_applies_filters",
    )
    .await;
    let pool = test_db.pool().clone();
    let filter = LegislativeInitiativeFilter {
        legis_period: Some("XXV".to_string()),
        accepted: Some("true".to_string()),
    };

    let Json(results) =
        legislative_initiatives_without_simple_majority(PgPoolConnection(pool), Json(Some(filter)))
            .await
            .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].total_initiatives, 1);
}
