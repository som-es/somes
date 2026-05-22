use super::*;

fn create_test_base_data() -> Vec<ComplexityBase> {
    vec![
        ComplexityBase {
            delegate_name: "Delegate A".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: "M".to_string(),
            complexity_score: 1.2,
            total_proposals: 10,
            legislative_period: Some("XXV".to_string()),
            delegate_age_bucket: "41-50".to_string(),
        },
        ComplexityBase {
            delegate_name: "Delegate B".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: "F".to_string(),
            complexity_score: 1.0,
            total_proposals: 5,
            legislative_period: Some("XXV".to_string()),
            delegate_age_bucket: "31-40".to_string(),
        },
        ComplexityBase {
            delegate_name: "Delegate C".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: "M".to_string(),
            complexity_score: 1.3,
            total_proposals: 8,
            legislative_period: Some("XXV".to_string()),
            delegate_age_bucket: "51-60".to_string(),
        },
        ComplexityBase {
            delegate_name: "Delegate D".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: "F".to_string(),
            complexity_score: 1.1,
            total_proposals: 12,
            legislative_period: Some("XXVII".to_string()),
            delegate_age_bucket: "41-50".to_string(),
        },
    ]
}

#[test]
fn test_aggregate_by_party() {
    let base_data = create_test_base_data();
    let results = ComplexityService::aggregate_by_party(base_data, true);

    // Verify Party X: (1.2 + 1.0) / 2 = 1.1, total_proposals = 15, delegate_count = 2
    let party_x = results.iter().find(|r| r.category == "Party X").unwrap();
    assert!((party_x.average_complexity - 1.1).abs() < 0.001);
    assert_eq!(party_x.total_proposals, 15);
    assert_eq!(party_x.delegate_count, 2);

    // Verify Party Y: (1.3 + 1.1) / 2 = 1.2, total_proposals = 20, delegate_count = 2
    let party_y = results.iter().find(|r| r.category == "Party Y").unwrap();
    assert!((party_y.average_complexity - 1.2).abs() < 0.001);
    assert_eq!(party_y.total_proposals, 20);
    assert_eq!(party_y.delegate_count, 2);
}

#[test]
fn test_aggregate_by_party_desc_false() {
    let base_data = create_test_base_data();
    let results = ComplexityService::aggregate_by_party(base_data, false);

    // When is_desc is false, results should be in ascending order
    assert!(results[0].average_complexity < results[1].average_complexity);
}

#[test]
fn test_aggregate_by_gender() {
    let base_data = create_test_base_data();
    let results = ComplexityService::aggregate_by_gender(base_data, true);

    // Verify M: (1.2 + 1.3) / 2 = 1.25, total_proposals = 18, delegate_count = 2
    let male = results.iter().find(|r| r.category == "M").unwrap();
    assert!((male.average_complexity - 1.25).abs() < 0.001);
    assert_eq!(male.total_proposals, 18);
    assert_eq!(male.delegate_count, 2);

    // Verify F: (1.0 + 1.1) / 2 = 1.05, total_proposals = 17, delegate_count = 2
    let female = results.iter().find(|r| r.category == "F").unwrap();
    assert!((female.average_complexity - 1.05).abs() < 0.001);
    assert_eq!(female.total_proposals, 17);
    assert_eq!(female.delegate_count, 2);
}

#[test]
fn test_aggregate_by_legis() {
    let base_data = create_test_base_data();
    let results = ComplexityService::aggregate_by_legis(base_data, true);

    // Verify 51: (1.2 + 1.0 + 1.3) / 3 = 1.166..., total_proposals = 23, delegate_count = 3
    let period_51 = results.iter().find(|r| r.category == "XXV").unwrap();
    assert!((period_51.average_complexity - 1.1666667).abs() < 0.001);
    assert_eq!(period_51.total_proposals, 23);
    assert_eq!(period_51.delegate_count, 3);

    // Verify 52: 1.1, total_proposals = 12, delegate_count = 1
    let period_52 = results.iter().find(|r| r.category == "XXVII").unwrap();
    assert!((period_52.average_complexity - 1.1).abs() < 0.001);
    assert_eq!(period_52.total_proposals, 12);
    assert_eq!(period_52.delegate_count, 1);
}

#[test]
fn test_aggregate_by_age() {
    let base_data = create_test_base_data();
    let results = ComplexityService::aggregate_by_age(base_data, true);

    // Verify 41-50: (1.2 + 1.1) / 2 = 1.15, total_proposals = 22, delegate_count = 2
    let age_41_50 = results.iter().find(|r| r.category == "41-50").unwrap();
    assert!((age_41_50.average_complexity - 1.15).abs() < 0.001);
    assert_eq!(age_41_50.total_proposals, 22);
    assert_eq!(age_41_50.delegate_count, 2);

    // Verify 31-40: 1.0, total_proposals = 5, delegate_count = 1
    let age_31_40 = results.iter().find(|r| r.category == "31-40").unwrap();
    assert!((age_31_40.average_complexity - 1.0).abs() < 0.001);
    assert_eq!(age_31_40.total_proposals, 5);
    assert_eq!(age_31_40.delegate_count, 1);

    // Verify 51-60: 1.3, total_proposals = 8, delegate_count = 1
    let age_51_60 = results.iter().find(|r| r.category == "51-60").unwrap();
    assert!((age_51_60.average_complexity - 1.3).abs() < 0.001);
    assert_eq!(age_51_60.total_proposals, 8);
    assert_eq!(age_51_60.delegate_count, 1);
}

#[tokio::test]
async fn test_get_base_data_applies_filters_and_computes_complexity_stats() {
    let test_db = super::super::test_db::statistics_test_db(
        "test_get_base_data_applies_filters_and_computes_complexity_stats",
    )
    .await;
    let pool = test_db.pool().clone();
    let filter = ComplexityFilter {
        legis_period: Some("XXV".to_string()),
        party: Some("Party X".to_string()),
        gender: Some("M".to_string()),
        ..Default::default()
    };

    let results = ComplexityService::get_base_data(&pool, &filter)
        .await
        .unwrap();

    assert_eq!(results.len(), 1);

    let delegate = &results[0];
    assert_eq!(delegate.delegate_name, "Delegate A");
    assert_eq!(delegate.delegate_party, "Party X");
    assert_eq!(delegate.delegate_gender, "M");
    assert!((delegate.complexity_score - 1.25).abs() < 0.001);
    assert_eq!(delegate.total_proposals, 2);
    assert_eq!(delegate.legislative_period, Some("XXV".to_string()));
    assert_eq!(delegate.delegate_age_bucket, "31-40");
}

#[tokio::test]
async fn test_per_legis_keeps_delegates_with_data_in_multiple_periods() {
    let test_db = super::super::test_db::statistics_test_db(
        "test_per_legis_keeps_delegates_with_data_in_multiple_periods",
    )
    .await;
    let pool = test_db.pool().clone();
    let filter = ComplexityFilter {
        is_desc: true,
        ..Default::default()
    };

    let results = ComplexityService::per_legis(&pool, &filter).await.unwrap();

    assert_eq!(results.len(), 3);

    let period_51 = results.iter().find(|r| r.category == "XXV").unwrap();
    assert!((period_51.average_complexity - 1.225).abs() < 0.001);
    assert_eq!(period_51.total_proposals, 3);
    assert_eq!(period_51.delegate_count, 2);

    let period_52 = results.iter().find(|r| r.category == "XXVII").unwrap();
    assert!((period_52.average_complexity - 1.05).abs() < 0.001);
    assert_eq!(period_52.total_proposals, 4);
    assert_eq!(period_52.delegate_count, 4);

    let period_53 = results.iter().find(|r| r.category == "XXVIII").unwrap();
    assert!((period_53.average_complexity - 1.225).abs() < 0.001);
    assert_eq!(period_53.total_proposals, 2);
    assert_eq!(period_53.delegate_count, 2);
}

#[tokio::test]
async fn test_per_delegate_aggregates_all_periods_into_one_delegate_row() {
    let test_db = super::super::test_db::statistics_test_db(
        "test_per_delegate_aggregates_all_periods_into_one_delegate_row",
    )
    .await;
    let pool = test_db.pool().clone();
    let filter = ComplexityFilter {
        is_desc: true,
        ..Default::default()
    };

    let results = ComplexityService::per_delegate(&pool, &filter)
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
    assert!((delegate.complexity_score - 1.1).abs() < 0.001);
    assert_eq!(delegate.total_proposals, 2);
}
