use sqlx::PgPool;

use super::*;

fn create_test_base_data() -> Vec<DivisionAccuracyBase> {
    vec![
        DivisionAccuracyBase {
            delegate_name: "Delegate A".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: "M".to_string(),
            accuracy_score: 0.8,
            total_votes: 10,
            latest_activity_date: None,
            delegate_age_bucket: "41-50".to_string(),
        },
        DivisionAccuracyBase {
            delegate_name: "Delegate B".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: "F".to_string(),
            accuracy_score: 0.6,
            total_votes: 5,
            latest_activity_date: None,
            delegate_age_bucket: "31-40".to_string(),
        },
        DivisionAccuracyBase {
            delegate_name: "Delegate C".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: "M".to_string(),
            accuracy_score: 0.9,
            total_votes: 8,
            latest_activity_date: None,
            delegate_age_bucket: "51-60".to_string(),
        },
        DivisionAccuracyBase {
            delegate_name: "Delegate D".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: "F".to_string(),
            accuracy_score: 0.7,
            total_votes: 12,
            latest_activity_date: None,
            delegate_age_bucket: "41-50".to_string(),
        },
    ]
}

#[test]
fn test_aggregate_by_party() {
    let base_data = create_test_base_data();
    let results = DivisionAccuracyService::aggregate_by_party(base_data, true);

    // Verify Party X: (0.8 + 0.6) / 2 = 0.7, total_votes = 15, delegate_count = 2
    let party_x = results.iter().find(|r| r.category == "Party X").unwrap();
    assert!((party_x.average_accuracy - 0.7).abs() < 0.001);
    assert_eq!(party_x.total_votes, 15);
    assert_eq!(party_x.delegate_count, 2);

    // Verify Party Y: (0.9 + 0.7) / 2 = 0.8, total_votes = 20, delegate_count = 2
    let party_y = results.iter().find(|r| r.category == "Party Y").unwrap();
    assert!((party_y.average_accuracy - 0.8).abs() < 0.001);
    assert_eq!(party_y.total_votes, 20);
    assert_eq!(party_y.delegate_count, 2);
}

#[test]
fn test_aggregate_by_party_desc_false() {
    let base_data = create_test_base_data();
    let results = DivisionAccuracyService::aggregate_by_party(base_data, false);

    assert!(results[0].average_accuracy < results[1].average_accuracy);
}

#[test]
fn test_aggregate_by_gender() {
    let base_data = create_test_base_data();
    let results = DivisionAccuracyService::aggregate_by_gender(base_data, true);

    // Verify M: (0.8 + 0.9) / 2 = 0.85, total_votes = 18, delegate_count = 2
    let male = results.iter().find(|r| r.category == "M").unwrap();
    assert!((male.average_accuracy - 0.85).abs() < 0.001);
    assert_eq!(male.total_votes, 18);
    assert_eq!(male.delegate_count, 2);

    // Verify F: (0.6 + 0.7) / 2 = 0.65, total_votes = 17, delegate_count = 2
    let female = results.iter().find(|r| r.category == "F").unwrap();
    assert!((female.average_accuracy - 0.65).abs() < 0.001);
    assert_eq!(female.total_votes, 17);
    assert_eq!(female.delegate_count, 2);
}

#[test]
fn test_aggregate_by_age() {
    let base_data = create_test_base_data();
    let results = DivisionAccuracyService::aggregate_by_age(base_data, true);

    // Verify 41-50: (0.8 + 0.7) / 2 = 0.75, total_votes = 22, delegate_count = 2
    let age_41_50 = results.iter().find(|r| r.category == "41-50").unwrap();
    assert!((age_41_50.average_accuracy - 0.75).abs() < 0.001);
    assert_eq!(age_41_50.total_votes, 22);
    assert_eq!(age_41_50.delegate_count, 2);

    // Verify 31-40: 0.6, total_votes = 5, delegate_count = 1
    let age_31_40 = results.iter().find(|r| r.category == "31-40").unwrap();
    assert!((age_31_40.average_accuracy - 0.6).abs() < 0.001);
    assert_eq!(age_31_40.total_votes, 5);
    assert_eq!(age_31_40.delegate_count, 1);

    // Verify 51-60: 0.9, total_votes = 8, delegate_count = 1
    let age_51_60 = results.iter().find(|r| r.category == "51-60").unwrap();
    assert!((age_51_60.average_accuracy - 0.9).abs() < 0.001);
    assert_eq!(age_51_60.total_votes, 8);
    assert_eq!(age_51_60.delegate_count, 1);
}

#[sqlx::test(fixtures("fixtures/statistics_base.sql"))]
async fn test_get_base_data_applies_filters_and_computes_division_accuracy_stats(pool: PgPool) {
    let filter = DivisionAccuracyFilter {
        legis_period: Some("XXV".to_string()),
        party: Some("Party X".to_string()),
        gender: Some("M".to_string()),
        ..Default::default()
    };

    let results = DivisionAccuracyService::get_base_data(&pool, &filter)
        .await
        .unwrap();

    assert_eq!(results.len(), 1);

    let delegate = &results[0];
    assert_eq!(delegate.delegate_name, "Delegate A");
    assert_eq!(delegate.delegate_party, "Party X");
    assert_eq!(delegate.delegate_gender, "M");
    assert!((delegate.accuracy_score - 0.5).abs() < 0.001);
    assert_eq!(delegate.total_votes, 2);
    assert_eq!(delegate.delegate_age_bucket, "31-40");
}

#[sqlx::test(fixtures("fixtures/statistics_base.sql"))]
async fn test_per_legis_averages_delegate_scores_not_raw_votes(pool: PgPool) {
    let filter = DivisionAccuracyFilter {
        is_desc: true,
        ..Default::default()
    };

    let results = DivisionAccuracyService::per_legis(&pool, &filter)
        .await
        .unwrap();

    assert_eq!(results.len(), 3);

    let period_51 = results.iter().find(|r| r.category == "XXV").unwrap();
    assert!((period_51.average_accuracy - 0.5).abs() < 0.001);
    assert_eq!(period_51.total_votes, 7);
    assert_eq!(period_51.delegate_count, 3);

    let period_52 = results.iter().find(|r| r.category == "XXVII").unwrap();
    assert!((period_52.average_accuracy - 0.625).abs() < 0.001);
    assert_eq!(period_52.total_votes, 5);
    assert_eq!(period_52.delegate_count, 4);

    let period_53 = results.iter().find(|r| r.category == "XXVIII").unwrap();
    assert!((period_53.average_accuracy - 0.5).abs() < 0.001);
    assert_eq!(period_53.total_votes, 2);
    assert_eq!(period_53.delegate_count, 2);
}

#[sqlx::test(fixtures("fixtures/statistics_base.sql"))]
async fn test_per_delegate_aggregates_party_period_rows_into_one_delegate_row(pool: PgPool) {
    let filter = DivisionAccuracyFilter {
        is_desc: true,
        ..Default::default()
    };

    let results = DivisionAccuracyService::per_delegate(&pool, &filter)
        .await
        .unwrap();

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
    assert!((delegate.accuracy_score - (1.0 / 6.0)).abs() < 0.001);
    assert_eq!(delegate.total_votes, 6);
}
