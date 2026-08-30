use sqlx::PgPool;

use super::*;

fn create_test_base_data() -> Vec<CallToOrdersBase> {
    vec![
        CallToOrdersBase {
            delegate_name: "Delegate A".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: Some("M".to_string()),
            total_order_calls: 10,
            total_sessions_attended: Some(20),
            normalized_calls_to_order: Some(0.5),
            legislative_period: Some("XXV".to_string()),
            delegate_age_bucket: "41-50".to_string(),
        },
        CallToOrdersBase {
            delegate_name: "Delegate B".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: Some("F".to_string()),
            total_order_calls: 5,
            total_sessions_attended: Some(20),
            normalized_calls_to_order: Some(0.25),
            legislative_period: Some("XXV".to_string()),
            delegate_age_bucket: "31-40".to_string(),
        },
        CallToOrdersBase {
            delegate_name: "Delegate C".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: Some("M".to_string()),
            total_order_calls: 15,
            total_sessions_attended: Some(20),
            normalized_calls_to_order: Some(0.75),
            legislative_period: Some("XXV".to_string()),
            delegate_age_bucket: "51-60".to_string(),
        },
        CallToOrdersBase {
            delegate_name: "Delegate D".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: Some("F".to_string()),
            total_order_calls: 8,
            total_sessions_attended: Some(20),
            normalized_calls_to_order: Some(0.4),
            legislative_period: Some("XXVII".to_string()),
            delegate_age_bucket: "41-50".to_string(),
        },
    ]
}

#[test]
fn test_aggregate_by_party() {
    let base_data = create_test_base_data();
    let results = CallToOrdersService::aggregate_by_party(base_data, true, false);

    // Verify Party X: total_calls = 15, total_sessions = 40, normalized = 15/40 = 0.375
    let party_x = results.iter().find(|r| r.category == "Party X").unwrap();
    assert_eq!(party_x.total_order_calls, 15);
    assert_eq!(party_x.total_sessions_attended, Some(40));
    assert!((party_x.normalized_calls_to_order.unwrap() - 0.375).abs() < 0.001);

    // Verify Party Y: total_calls = 23, total_sessions = 40, normalized = 23/40 = 0.575
    let party_y = results.iter().find(|r| r.category == "Party Y").unwrap();
    assert_eq!(party_y.total_order_calls, 23);
    assert_eq!(party_y.total_sessions_attended, Some(40));
    assert!((party_y.normalized_calls_to_order.unwrap() - 0.575).abs() < 0.001);
}

#[test]
fn test_aggregate_by_party_desc_false() {
    let base_data = create_test_base_data();
    let results = CallToOrdersService::aggregate_by_party(base_data, false, false);

    assert!(results[0].total_order_calls < results[1].total_order_calls);
}

#[test]
fn test_aggregate_by_gender() {
    let base_data = create_test_base_data();
    let results = CallToOrdersService::aggregate_by_gender(base_data, true, false);

    // Verify M: total_calls = 25, total_sessions = 40, normalized = 25/40 = 0.625
    let male = results.iter().find(|r| r.category == "M").unwrap();
    assert_eq!(male.total_order_calls, 25);
    assert_eq!(male.total_sessions_attended, Some(40));
    assert!((male.normalized_calls_to_order.unwrap() - 0.625).abs() < 0.001);

    // Verify F: total_calls = 13, total_sessions = 40, normalized = 13/40 = 0.325
    let female = results.iter().find(|r| r.category == "F").unwrap();
    assert_eq!(female.total_order_calls, 13);
    assert_eq!(female.total_sessions_attended, Some(40));
    assert!((female.normalized_calls_to_order.unwrap() - 0.325).abs() < 0.001);
}

#[test]
fn test_aggregate_by_legis() {
    let base_data = create_test_base_data();
    let results = CallToOrdersService::aggregate_by_legis(base_data, true, false);

    // Verify 51: total_calls = 30, total_sessions = 60, normalized = 30/60 = 0.5
    let period_51 = results.iter().find(|r| r.category == "XXV").unwrap();
    assert_eq!(period_51.total_order_calls, 30);
    assert_eq!(period_51.total_sessions_attended, Some(60));
    assert!((period_51.normalized_calls_to_order.unwrap() - 0.5).abs() < 0.001);

    // Verify 52: total_calls = 8, total_sessions = 20, normalized = 8/20 = 0.4
    let period_52 = results.iter().find(|r| r.category == "XXVII").unwrap();
    assert_eq!(period_52.total_order_calls, 8);
    assert_eq!(period_52.total_sessions_attended, Some(20));
    assert!((period_52.normalized_calls_to_order.unwrap() - 0.4).abs() < 0.001);
}

#[test]
fn test_aggregate_by_age() {
    let base_data = create_test_base_data();
    let results = CallToOrdersService::aggregate_by_age(base_data, true, false);

    // Verify 41-50: total_calls = 18, total_sessions = 40, normalized = 18/40 = 0.45
    let age_41_50 = results.iter().find(|r| r.category == "41-50").unwrap();
    assert_eq!(age_41_50.total_order_calls, 18);
    assert_eq!(age_41_50.total_sessions_attended, Some(40));
    assert!((age_41_50.normalized_calls_to_order.unwrap() - 0.45).abs() < 0.001);

    // Verify 31-40: total_calls = 5, total_sessions = 20, normalized = 5/20 = 0.25
    let age_31_40 = results.iter().find(|r| r.category == "31-40").unwrap();
    assert_eq!(age_31_40.total_order_calls, 5);
    assert_eq!(age_31_40.total_sessions_attended, Some(20));
    assert!((age_31_40.normalized_calls_to_order.unwrap() - 0.25).abs() < 0.001);

    // Verify 51-60: total_calls = 15, total_sessions = 20, normalized = 15/20 = 0.75
    let age_51_60 = results.iter().find(|r| r.category == "51-60").unwrap();
    assert_eq!(age_51_60.total_order_calls, 15);
    assert_eq!(age_51_60.total_sessions_attended, Some(20));
    assert!((age_51_60.normalized_calls_to_order.unwrap() - 0.75).abs() < 0.001);
}

#[test]
fn test_aggregate_by_party_sorts_by_normalized_score() {
    let base_data = vec![
        CallToOrdersBase {
            delegate_name: "Delegate A".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: Some("M".to_string()),
            total_order_calls: 10,
            total_sessions_attended: Some(100),
            normalized_calls_to_order: Some(0.1),
            legislative_period: Some("XXV".to_string()),
            delegate_age_bucket: "41-50".to_string(),
        },
        CallToOrdersBase {
            delegate_name: "Delegate B".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: Some("F".to_string()),
            total_order_calls: 2,
            total_sessions_attended: Some(4),
            normalized_calls_to_order: Some(0.5),
            legislative_period: Some("XXV".to_string()),
            delegate_age_bucket: "31-40".to_string(),
        },
    ];

    let results = CallToOrdersService::aggregate_by_party(base_data, true, true);

    assert_eq!(results[0].category, "Party Y");
}

#[sqlx::test(fixtures("fixtures/statistics_base.sql"))]
async fn test_get_base_data_applies_filters_and_computes_call_to_order_stats(pool: PgPool) {
    let filter = CallToOrderFilter {
        legis_period: Some("XXV".to_string()),
        party: Some("Party X".to_string()),
        gender: Some("M".to_string()),
        ..Default::default()
    };

    let results = CallToOrdersService::get_base_data(&pool, &filter)
        .await
        .unwrap();

    assert_eq!(results.len(), 1);

    let delegate = &results[0];
    assert_eq!(delegate.delegate_name, "Delegate A");
    assert_eq!(delegate.delegate_party, "Party X");
    assert_eq!(delegate.delegate_gender, Some("M".to_string()));
    assert_eq!(delegate.total_order_calls, 2);
    assert_eq!(delegate.total_sessions_attended, Some(1));
    assert!((delegate.normalized_calls_to_order.unwrap() - 2.0).abs() < 0.001);
    assert_eq!(delegate.legislative_period, Some("XXV".to_string()));
    assert_eq!(delegate.delegate_age_bucket, "31-40");
}

#[sqlx::test(fixtures("fixtures/statistics_base.sql"))]
async fn test_per_legis_keeps_delegates_with_data_in_multiple_periods(pool: PgPool) {
    let filter = CallToOrderFilter {
        is_desc: true,
        ..Default::default()
    };

    let results = CallToOrdersService::per_legis(&pool, &filter)
        .await
        .unwrap();

    assert_eq!(results.len(), 3);

    let period_51 = results.iter().find(|r| r.category == "XXV").unwrap();
    assert_eq!(period_51.total_order_calls, 4);
    assert_eq!(period_51.total_sessions_attended, Some(3));
    assert!((period_51.normalized_calls_to_order.unwrap() - (4.0 / 3.0)).abs() < 0.001);

    let period_52 = results.iter().find(|r| r.category == "XXVII").unwrap();
    assert_eq!(period_52.total_order_calls, 5);
    assert_eq!(period_52.total_sessions_attended, Some(3));
    assert!((period_52.normalized_calls_to_order.unwrap() - (5.0 / 3.0)).abs() < 0.001);

    let period_53 = results.iter().find(|r| r.category == "XXVIII").unwrap();
    assert_eq!(period_53.total_order_calls, 2);
    assert_eq!(period_53.total_sessions_attended, Some(2));
    assert!((period_53.normalized_calls_to_order.unwrap() - 1.0).abs() < 0.001);
}

#[sqlx::test(fixtures("fixtures/statistics_base.sql"))]
async fn test_per_delegate_aggregates_all_periods_into_one_delegate_row(pool: PgPool) {
    let filter = CallToOrderFilter {
        is_desc: true,
        ..Default::default()
    };

    let results = CallToOrdersService::per_delegate(&pool, &filter)
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
    assert_eq!(delegate.total_order_calls, 4);
    assert_eq!(delegate.total_sessions_attended, 2);
    assert!((delegate.normalized_calls_to_order - 2.0).abs() < 0.001);
}
