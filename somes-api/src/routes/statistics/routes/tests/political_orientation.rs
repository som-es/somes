use super::*;

fn create_orientation_base_data() -> Vec<PoliticalOrientationBase> {
    vec![
        PoliticalOrientationBase {
            delegate_name: "Delegate A".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: "M".to_string(),
            orientation_score: 0.8,
            total_votes: 10,
            delegate_age_bucket: "41-50".to_string(),
        },
        PoliticalOrientationBase {
            delegate_name: "Delegate B".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: "F".to_string(),
            orientation_score: 0.6,
            total_votes: 5,
            delegate_age_bucket: "31-40".to_string(),
        },
        PoliticalOrientationBase {
            delegate_name: "Delegate C".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: "M".to_string(),
            orientation_score: 0.9,
            total_votes: 8,
            delegate_age_bucket: "51-60".to_string(),
        },
        PoliticalOrientationBase {
            delegate_name: "Delegate D".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: "F".to_string(),
            orientation_score: 0.7,
            total_votes: 12,
            delegate_age_bucket: "41-50".to_string(),
        },
    ]
}

fn create_spectrum_base_data() -> Vec<PoliticalSpectrumBase> {
    vec![
        PoliticalSpectrumBase {
            delegate_name: "Delegate A".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: "M".to_string(),
            left_right_score: -0.4,
            liberal_authoritarian_score: -0.2,
            total_votes: 10,
            delegate_age_bucket: "41-50".to_string(),
        },
        PoliticalSpectrumBase {
            delegate_name: "Delegate B".to_string(),
            delegate_party: "Party X".to_string(),
            delegate_filter_party: "Party X".to_string(),
            delegate_gender: "F".to_string(),
            left_right_score: -0.2,
            liberal_authoritarian_score: -0.4,
            total_votes: 5,
            delegate_age_bucket: "31-40".to_string(),
        },
        PoliticalSpectrumBase {
            delegate_name: "Delegate C".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: "M".to_string(),
            left_right_score: 0.8,
            liberal_authoritarian_score: 0.6,
            total_votes: 8,
            delegate_age_bucket: "51-60".to_string(),
        },
        PoliticalSpectrumBase {
            delegate_name: "Delegate D".to_string(),
            delegate_party: "Party Y".to_string(),
            delegate_filter_party: "Party Y".to_string(),
            delegate_gender: "F".to_string(),
            left_right_score: 0.6,
            liberal_authoritarian_score: 0.8,
            total_votes: 12,
            delegate_age_bucket: "41-50".to_string(),
        },
    ]
}

#[test]
fn test_orientation_aggregate_by_party() {
    let base_data = create_orientation_base_data();
    let results = PoliticalOrientationService::aggregate_by_party(base_data, true);

    // Verify Party X: (0.8 + 0.6) / 2 = 0.7, total_votes = 15, delegate_count = 2
    let party_x = results.iter().find(|r| r.category == "Party X").unwrap();
    assert!((party_x.average_orientation - 0.7).abs() < 0.001);
    assert_eq!(party_x.total_votes, 15);
    assert_eq!(party_x.delegate_count, 2);

    // Verify Party Y: (0.9 + 0.7) / 2 = 0.8, total_votes = 20, delegate_count = 2
    let party_y = results.iter().find(|r| r.category == "Party Y").unwrap();
    assert!((party_y.average_orientation - 0.8).abs() < 0.001);
    assert_eq!(party_y.total_votes, 20);
    assert_eq!(party_y.delegate_count, 2);
}

#[test]
fn test_orientation_aggregate_by_party_desc_false() {
    let base_data = create_orientation_base_data();
    let results = PoliticalOrientationService::aggregate_by_party(base_data, false);

    assert!(results[0].average_orientation < results[1].average_orientation);
}

#[test]
fn test_orientation_aggregate_by_gender() {
    let base_data = create_orientation_base_data();
    let results = PoliticalOrientationService::aggregate_by_gender(base_data, true);

    // Verify M: (0.8 + 0.9) / 2 = 0.85, total_votes = 18, delegate_count = 2
    let male = results.iter().find(|r| r.category == "M").unwrap();
    assert!((male.average_orientation - 0.85).abs() < 0.001);
    assert_eq!(male.total_votes, 18);
    assert_eq!(male.delegate_count, 2);

    // Verify F: (0.6 + 0.7) / 2 = 0.65, total_votes = 17, delegate_count = 2
    let female = results.iter().find(|r| r.category == "F").unwrap();
    assert!((female.average_orientation - 0.65).abs() < 0.001);
    assert_eq!(female.total_votes, 17);
    assert_eq!(female.delegate_count, 2);
}

#[test]
fn test_orientation_aggregate_by_age() {
    let base_data = create_orientation_base_data();
    let results = PoliticalOrientationService::aggregate_by_age(base_data, true);

    // Verify 41-50: (0.8 + 0.7) / 2 = 0.75, total_votes = 22, delegate_count = 2
    let age_41_50 = results.iter().find(|r| r.category == "41-50").unwrap();
    assert!((age_41_50.average_orientation - 0.75).abs() < 0.001);
    assert_eq!(age_41_50.total_votes, 22);
    assert_eq!(age_41_50.delegate_count, 2);

    // Verify 31-40: 0.6, total_votes = 5, delegate_count = 1
    let age_31_40 = results.iter().find(|r| r.category == "31-40").unwrap();
    assert!((age_31_40.average_orientation - 0.6).abs() < 0.001);
    assert_eq!(age_31_40.total_votes, 5);
    assert_eq!(age_31_40.delegate_count, 1);

    // Verify 51-60: 0.9, total_votes = 8, delegate_count = 1
    let age_51_60 = results.iter().find(|r| r.category == "51-60").unwrap();
    assert!((age_51_60.average_orientation - 0.9).abs() < 0.001);
    assert_eq!(age_51_60.total_votes, 8);
    assert_eq!(age_51_60.delegate_count, 1);
}

#[test]
fn test_spectrum_aggregate_by_party() {
    let base_data = create_spectrum_base_data();
    let results = PoliticalSpectrumService::aggregate_by_party(base_data, true);

    // Verify Party X: left/right = -0.3, liberal/authoritarian = -0.3
    let party_x = results.iter().find(|r| r.category == "Party X").unwrap();
    assert!((party_x.average_left_right_score + 0.3).abs() < 0.001);
    assert!((party_x.average_liberal_authoritarian_score + 0.3).abs() < 0.001);
    assert!((party_x.spectrum_magnitude - 0.424264).abs() < 0.001);
    assert_eq!(party_x.total_votes, 15);
    assert_eq!(party_x.delegate_count, 2);

    // Verify Party Y: left/right = 0.7, liberal/authoritarian = 0.7
    let party_y = results.iter().find(|r| r.category == "Party Y").unwrap();
    assert!((party_y.average_left_right_score - 0.7).abs() < 0.001);
    assert!((party_y.average_liberal_authoritarian_score - 0.7).abs() < 0.001);
    assert!((party_y.spectrum_magnitude - 0.989949).abs() < 0.001);
    assert_eq!(party_y.total_votes, 20);
    assert_eq!(party_y.delegate_count, 2);
}

#[test]
fn test_spectrum_aggregate_by_party_desc_false() {
    let base_data = create_spectrum_base_data();
    let results = PoliticalSpectrumService::aggregate_by_party(base_data, false);

    assert!(results[0].spectrum_magnitude < results[1].spectrum_magnitude);
}

#[test]
fn test_spectrum_aggregate_by_gender() {
    let base_data = create_spectrum_base_data();
    let results = PoliticalSpectrumService::aggregate_by_gender(base_data, true);

    // Verify M: left/right = 0.2, liberal/authoritarian = 0.2
    let male = results.iter().find(|r| r.category == "M").unwrap();
    assert!((male.average_left_right_score - 0.2).abs() < 0.001);
    assert!((male.average_liberal_authoritarian_score - 0.2).abs() < 0.001);
    assert_eq!(male.total_votes, 18);
    assert_eq!(male.delegate_count, 2);

    // Verify F: left/right = 0.2, liberal/authoritarian = 0.2
    let female = results.iter().find(|r| r.category == "F").unwrap();
    assert!((female.average_left_right_score - 0.2).abs() < 0.001);
    assert!((female.average_liberal_authoritarian_score - 0.2).abs() < 0.001);
    assert_eq!(female.total_votes, 17);
    assert_eq!(female.delegate_count, 2);
}

#[test]
fn test_spectrum_aggregate_by_age() {
    let base_data = create_spectrum_base_data();
    let results = PoliticalSpectrumService::aggregate_by_age(base_data, true);

    // Verify 41-50: left/right = 0.1, liberal/authoritarian = 0.3
    let age_41_50 = results.iter().find(|r| r.category == "41-50").unwrap();
    assert!((age_41_50.average_left_right_score - 0.1).abs() < 0.001);
    assert!((age_41_50.average_liberal_authoritarian_score - 0.3).abs() < 0.001);
    assert_eq!(age_41_50.total_votes, 22);
    assert_eq!(age_41_50.delegate_count, 2);

    // Verify 31-40: left/right = -0.2, liberal/authoritarian = -0.4
    let age_31_40 = results.iter().find(|r| r.category == "31-40").unwrap();
    assert!((age_31_40.average_left_right_score + 0.2).abs() < 0.001);
    assert!((age_31_40.average_liberal_authoritarian_score + 0.4).abs() < 0.001);
    assert_eq!(age_31_40.total_votes, 5);
    assert_eq!(age_31_40.delegate_count, 1);

    // Verify 51-60: left/right = 0.8, liberal/authoritarian = 0.6
    let age_51_60 = results.iter().find(|r| r.category == "51-60").unwrap();
    assert!((age_51_60.average_left_right_score - 0.8).abs() < 0.001);
    assert!((age_51_60.average_liberal_authoritarian_score - 0.6).abs() < 0.001);
    assert_eq!(age_51_60.total_votes, 8);
    assert_eq!(age_51_60.delegate_count, 1);
}

#[sqlx::test(migrations = false, fixtures("./fixtures/statistics_base.sql"))]
async fn test_orientation_get_base_data_applies_filters(pool: sqlx::PgPool) {
    let filter = PoliticalOrientationFilter {
        legis_period: Some("51".to_string()),
        party: Some("Party X".to_string()),
        gender: Some("M".to_string()),
        orientation_type: "left".to_string(),
        ..Default::default()
    };

    let results = PoliticalOrientationService::get_base_data(&pool, &filter)
        .await
        .unwrap();

    assert_eq!(results.len(), 1);

    let delegate = &results[0];
    assert_eq!(delegate.delegate_name, "Delegate A");
    assert_eq!(delegate.delegate_party, "Party X");
    assert_eq!(delegate.delegate_gender, "M");
    assert!((delegate.orientation_score - 0.75).abs() < 0.001);
    assert_eq!(delegate.total_votes, 20);
    assert_eq!(delegate.delegate_age_bucket, "31-40");
}

#[sqlx::test(migrations = false, fixtures("./fixtures/statistics_base.sql"))]
async fn test_orientation_per_legis_averages_active_delegates_in_period(pool: sqlx::PgPool) {
    let filter = PoliticalOrientationFilter {
        legis_period: Some("51".to_string()),
        orientation_type: "left".to_string(),
        ..Default::default()
    };

    let results = PoliticalOrientationService::per_legis(&pool, &filter)
        .await
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].category, "51");
    assert!((results[0].average_orientation - 0.5).abs() < 0.001);
    assert_eq!(results[0].total_votes, 42);
    assert_eq!(results[0].delegate_count, 3);
}

#[sqlx::test(migrations = false, fixtures("./fixtures/statistics_base.sql"))]
async fn test_orientation_handlers_override_orientation_type(pool: sqlx::PgPool) {
    let filter = PoliticalOrientationFilter {
        legis_period: Some("51".to_string()),
        party: Some("Party X".to_string()),
        gender: Some("M".to_string()),
        orientation_type: "left".to_string(),
        ..Default::default()
    };

    let Json(right_results) =
        is_right_per_delegate(PgPoolConnection(pool.clone()), Json(Some(filter.clone())))
            .await
            .unwrap();
    assert_eq!(right_results.len(), 1);
    assert_eq!(right_results[0].delegate_name, "Delegate A");
    assert!((right_results[0].orientation_score - 0.25).abs() < 0.001);

    let Json(liberal_results) =
        is_liberal_per_delegate(PgPoolConnection(pool.clone()), Json(Some(filter.clone())))
            .await
            .unwrap();
    assert_eq!(liberal_results.len(), 1);
    assert_eq!(liberal_results[0].delegate_name, "Delegate A");
    assert!((liberal_results[0].orientation_score - 0.6).abs() < 0.001);

    let Json(authoritarian_results) =
        is_authoritarian_per_delegate(PgPoolConnection(pool), Json(Some(filter)))
            .await
            .unwrap();
    assert_eq!(authoritarian_results.len(), 1);
    assert_eq!(authoritarian_results[0].delegate_name, "Delegate A");
    assert!((authoritarian_results[0].orientation_score - 0.4).abs() < 0.001);
}

#[sqlx::test(migrations = false, fixtures("./fixtures/statistics_base.sql"))]
async fn test_spectrum_get_base_data_applies_filters(pool: sqlx::PgPool) {
    let filter = PoliticalOrientationFilter {
        legis_period: Some("51".to_string()),
        party: Some("Party X".to_string()),
        gender: Some("M".to_string()),
        ..Default::default()
    };

    let results = PoliticalSpectrumService::get_base_data(&pool, &filter)
        .await
        .unwrap();

    assert_eq!(results.len(), 1);

    let delegate = &results[0];
    assert_eq!(delegate.delegate_name, "Delegate A");
    assert_eq!(delegate.delegate_party, "Party X");
    assert_eq!(delegate.delegate_gender, "M");
    assert!((delegate.left_right_score + 0.5).abs() < 0.001);
    assert!((delegate.liberal_authoritarian_score + 0.2).abs() < 0.001);
    assert_eq!(delegate.total_votes, 20);
    assert_eq!(delegate.delegate_age_bucket, "31-40");
}

#[sqlx::test(migrations = false, fixtures("./fixtures/statistics_base.sql"))]
async fn test_votes_together_applies_legislative_period_filter(pool: sqlx::PgPool) {
    let filter = VotesTogetherFilter {
        legis_period: Some("51".to_string()),
        ..Default::default()
    };

    let Json(results) = votes_together(PgPoolConnection(pool), Json(Some(filter)))
        .await
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].party_1, "Party X");
    assert_eq!(results[0].party_2, "Party Y");
    assert_eq!(results[0].same_votes, 2);
}
