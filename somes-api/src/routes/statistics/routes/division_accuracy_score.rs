use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres};
use utoipa::ToSchema;

use crate::{
    routes::statistics::routes::error::StatisticsResponse,
    routes::statistics::routes::filtering::{
        bind_values, build_filter, IntoFilterArgument, Manual,
    },
    PgPoolConnection,
};

#[derive(ToSchema, Default, Debug, Clone, Serialize, Deserialize)]
pub struct DivisionAccuracyFilter {
    legis_period: Option<String>,
    gender: Option<String>,
    party: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DivisionAccuracyBase {
    delegate_name: String,
    delegate_party: String,
    delegate_filter_party: String,
    delegate_gender: Option<String>,
    accuracy_score: f64,
    total_votes: i64,
    latest_activity_date: Option<chrono::NaiveDate>,
    delegate_age_bucket: String,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DivisionAccuracyForDelegate {
    delegate_name: String,
    delegate_party: String,
    delegate_filter_party: String,
    accuracy_score: f64,
    total_votes: i64,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DivisionAccuracyByCategory {
    category: String,
    average_accuracy: f64,
    total_votes: i64,
    delegate_count: i64,
}

pub struct DivisionAccuracyService;

impl DivisionAccuracyService {
    fn sort_categories(results: &mut [DivisionAccuracyByCategory], is_desc: bool) {
        results.sort_by(|a, b| {
            b.average_accuracy
                .partial_cmp(&a.average_accuracy)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !is_desc {
            results.reverse();
        }
    }

    fn aggregate_by_party(
        base_data: Vec<DivisionAccuracyBase>,
        is_desc: bool,
    ) -> Vec<DivisionAccuracyByCategory> {
        let mut party_map: std::collections::HashMap<String, (Vec<f64>, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry = party_map
                .entry(item.delegate_filter_party.clone())
                .or_insert((Vec::new(), 0, 0));
            entry.0.push(item.accuracy_score);
            entry.1 += item.total_votes;
            entry.2 += 1;
        }

        let mut results: Vec<DivisionAccuracyByCategory> = party_map
            .into_iter()
            .map(|(party, (scores, total_votes, delegate_count))| {
                let average_accuracy = if !scores.is_empty() {
                    scores.iter().sum::<f64>() / scores.len() as f64
                } else {
                    0.0
                };

                DivisionAccuracyByCategory {
                    category: party,
                    average_accuracy,
                    total_votes,
                    delegate_count,
                }
            })
            .collect();

        Self::sort_categories(&mut results, is_desc);
        results
    }

    fn aggregate_by_gender(
        base_data: Vec<DivisionAccuracyBase>,
        is_desc: bool,
    ) -> Vec<DivisionAccuracyByCategory> {
        let mut gender_map: std::collections::HashMap<String, (Vec<f64>, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry =
                gender_map
                    .entry(
                        item.delegate_gender
                            .clone()
                            .unwrap_or_else(|| "Unknown".into()),
                    )
                    .or_insert((Vec::new(), 0, 0));
            entry.0.push(item.accuracy_score);
            entry.1 += item.total_votes;
            entry.2 += 1;
        }

        let mut results: Vec<DivisionAccuracyByCategory> = gender_map
            .into_iter()
            .map(|(gender, (scores, total_votes, delegate_count))| {
                let average_accuracy = if !scores.is_empty() {
                    scores.iter().sum::<f64>() / scores.len() as f64
                } else {
                    0.0
                };

                DivisionAccuracyByCategory {
                    category: gender,
                    average_accuracy,
                    total_votes,
                    delegate_count,
                }
            })
            .collect();

        Self::sort_categories(&mut results, is_desc);
        results
    }

    fn aggregate_by_age(
        base_data: Vec<DivisionAccuracyBase>,
        is_desc: bool,
    ) -> Vec<DivisionAccuracyByCategory> {
        let mut age_map: std::collections::HashMap<String, (Vec<f64>, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry = age_map
                .entry(item.delegate_age_bucket)
                .or_insert((Vec::new(), 0, 0));
            entry.0.push(item.accuracy_score);
            entry.1 += item.total_votes;
            entry.2 += 1;
        }

        let mut results: Vec<DivisionAccuracyByCategory> = age_map
            .into_iter()
            .map(|(category, (scores, total_votes, delegate_count))| {
                let average_accuracy = if !scores.is_empty() {
                    scores.iter().sum::<f64>() / scores.len() as f64
                } else {
                    0.0
                };

                DivisionAccuracyByCategory {
                    category,
                    average_accuracy,
                    total_votes,
                    delegate_count,
                }
            })
            .collect();

        Self::sort_categories(&mut results, is_desc);
        results
    }

    pub async fn get_base_data(
        pg: &sqlx::PgPool,
        filter: &DivisionAccuracyFilter,
    ) -> Result<Vec<DivisionAccuracyBase>, StatisticsResponse> {
        let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
        let filter_arg1 = filter
            .party
            .with_sql_column("COALESCE(m.party, 'Regierungsmitglied')");
        let filter_arg2 = filter.gender.with_sql_column("d.gender");
        let filter_arg3 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
        let filters = [filter_arg, filter_arg1, filter_arg2, filter_arg3];

        let filter_str = build_filter(&filters);

        let query = format!(
            "
        SELECT
            d.name AS delegate_name,
            COALESCE(m.party, d.party, 'Regierungsmitglied') AS delegate_party,
            COALESCE(m.party, 'Regierungsmitglied') AS delegate_filter_party,
            d.gender AS delegate_gender,
            AVG(CASE WHEN dv.vote = dv.outcome THEN 1.0::float8 ELSE 0.0::float8 END)::float8 AS accuracy_score,
            COUNT(dv.id) AS total_votes,
            MAX(pf.raw_data_created_at)::date AS latest_activity_date,
            CASE
                WHEN d.birthdate IS NULL THEN 'Unbekannt'
                WHEN EXTRACT(YEAR FROM AGE(COALESCE(MAX(pf.raw_data_created_at)::date, CURRENT_DATE), d.birthdate)) <= 30 THEN '18-30'
                WHEN EXTRACT(YEAR FROM AGE(COALESCE(MAX(pf.raw_data_created_at)::date, CURRENT_DATE), d.birthdate)) <= 40 THEN '31-40'
                WHEN EXTRACT(YEAR FROM AGE(COALESCE(MAX(pf.raw_data_created_at)::date, CURRENT_DATE), d.birthdate)) <= 50 THEN '41-50'
                WHEN EXTRACT(YEAR FROM AGE(COALESCE(MAX(pf.raw_data_created_at)::date, CURRENT_DATE), d.birthdate)) <= 60 THEN '51-60'
                ELSE '60+'
            END AS delegate_age_bucket
        FROM
            delegate_votes dv
        JOIN
            delegates d ON dv.delegate_id = d.id
        LEFT JOIN plenar_infos pf ON pf.id = dv.plenar_id
        LEFT JOIN mandates m ON m.delegate_id = d.id
            AND (m.start_date IS NULL OR m.start_date <= pf.raw_data_created_at::date)
            AND (m.end_date IS NULL OR m.end_date >= pf.raw_data_created_at::date)
        WHERE
            dv.outcome IS NOT NULL
            AND {filter_str}
        GROUP BY
            d.id, d.name, d.party, m.party, d.gender, d.birthdate
        ORDER BY
            accuracy_score DESC;
        "
        );

        let mut filtered_query = sqlx::query_as::<Postgres, DivisionAccuracyBase>(&query);
        filtered_query = bind_values(filtered_query, &filters);

        filtered_query
            .fetch_all(pg)
            .await
            .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
    }

    pub async fn per_delegate(
        pg: &sqlx::PgPool,
        filter: &DivisionAccuracyFilter,
    ) -> Result<Vec<DivisionAccuracyForDelegate>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;

        struct DelegateAccumulator {
            delegate_party: String,
            delegate_filter_party: String,
            weighted_accuracy: f64,
            total_votes: i64,
            latest_activity_date: Option<chrono::NaiveDate>,
        }

        let mut delegate_map: std::collections::HashMap<String, DelegateAccumulator> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry =
                delegate_map
                    .entry(item.delegate_name)
                    .or_insert_with(|| DelegateAccumulator {
                        delegate_party: item.delegate_party.clone(),
                        delegate_filter_party: item.delegate_filter_party.clone(),
                        weighted_accuracy: 0.0,
                        total_votes: 0,
                        latest_activity_date: None,
                    });

            entry.weighted_accuracy += item.accuracy_score * item.total_votes as f64;
            entry.total_votes += item.total_votes;

            if item.latest_activity_date >= entry.latest_activity_date {
                entry.delegate_party = item.delegate_party;
                entry.delegate_filter_party = item.delegate_filter_party;
                entry.latest_activity_date = item.latest_activity_date;
            }
        }

        let mut results: Vec<DivisionAccuracyForDelegate> = delegate_map
            .into_iter()
            .map(|(delegate_name, item)| {
                let accuracy_score = if item.total_votes > 0 {
                    item.weighted_accuracy / item.total_votes as f64
                } else {
                    0.0
                };

                DivisionAccuracyForDelegate {
                    delegate_name,
                    delegate_party: item.delegate_party,
                    delegate_filter_party: item.delegate_filter_party,
                    accuracy_score,
                    total_votes: item.total_votes,
                }
            })
            .collect();

        results.sort_by(|a, b| {
            b.accuracy_score
                .partial_cmp(&a.accuracy_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_party(
        pg: &sqlx::PgPool,
        filter: &DivisionAccuracyFilter,
    ) -> Result<Vec<DivisionAccuracyByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        Ok(Self::aggregate_by_party(base_data, filter.is_desc))
    }

    pub async fn per_gender(
        pg: &sqlx::PgPool,
        filter: &DivisionAccuracyFilter,
    ) -> Result<Vec<DivisionAccuracyByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        Ok(Self::aggregate_by_gender(base_data, filter.is_desc))
    }

    pub async fn per_legis(
        pg: &sqlx::PgPool,
        filter: &DivisionAccuracyFilter,
    ) -> Result<Vec<DivisionAccuracyByCategory>, StatisticsResponse> {
        let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
        let filter_arg1 = filter.gender.with_sql_column("d.gender");
        let filter_arg2 = filter
            .party
            .with_sql_column("COALESCE(m.party, 'Regierungsmitglied')");
        let filter_arg3 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
        let filters = [filter_arg, filter_arg1, filter_arg2, filter_arg3];

        let filter_str = build_filter(&filters);

        let query = format!(
            "
        WITH delegate_period_accuracy AS (
            SELECT
                pf.legislative_period AS category,
                d.id AS delegate_id,
                AVG(CASE WHEN dv.vote = dv.outcome THEN 1.0::float8 ELSE 0.0::float8 END)::float8 AS accuracy_score,
                COUNT(dv.id)::bigint AS total_votes
            FROM
                delegate_votes dv
            JOIN
                delegates d ON dv.delegate_id = d.id
            JOIN plenar_infos pf ON pf.id = dv.plenar_id
            LEFT JOIN mandates m ON m.delegate_id = d.id
                AND (m.start_date IS NULL OR m.start_date <= pf.raw_data_created_at::date)
                AND (m.end_date IS NULL OR m.end_date >= pf.raw_data_created_at::date)
            WHERE
                dv.outcome IS NOT NULL
                AND {filter_str}
            GROUP BY
                pf.legislative_period, d.id
        )
        SELECT
            category,
            AVG(accuracy_score)::float8 AS average_accuracy,
            SUM(total_votes)::bigint AS total_votes,
            COUNT(delegate_id)::bigint AS delegate_count
        FROM
            delegate_period_accuracy
        GROUP BY
            category
        ORDER BY
            average_accuracy DESC;
        "
        );

        let mut filtered_query = sqlx::query_as::<Postgres, DivisionAccuracyByCategory>(&query);
        filtered_query = bind_values(filtered_query, &filters);

        let mut results = filtered_query
            .fetch_all(pg)
            .await
            .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))?;

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_age(
        pg: &sqlx::PgPool,
        filter: &DivisionAccuracyFilter,
    ) -> Result<Vec<DivisionAccuracyByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        Ok(Self::aggregate_by_age(base_data, filter.is_desc))
    }
}

// Legacy endpoint functions for backward compatibility
pub async fn division_accuracy_score_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DivisionAccuracyFilter>>,
) -> Result<Json<Vec<DivisionAccuracyForDelegate>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = DivisionAccuracyService::per_delegate(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn division_accuracy_score_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DivisionAccuracyFilter>>,
) -> Result<Json<Vec<DivisionAccuracyByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = DivisionAccuracyService::per_party(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn division_accuracy_score_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DivisionAccuracyFilter>>,
) -> Result<Json<Vec<DivisionAccuracyByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = DivisionAccuracyService::per_gender(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn division_accuracy_score_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DivisionAccuracyFilter>>,
) -> Result<Json<Vec<DivisionAccuracyByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = DivisionAccuracyService::per_legis(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn division_accuracy_score_per_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DivisionAccuracyFilter>>,
) -> Result<Json<Vec<DivisionAccuracyByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = DivisionAccuracyService::per_age(&pg, &filter).await?;
    Ok(Json(results))
}

#[cfg(test)]
#[path = "tests/division_accuracy_score.rs"]
mod tests;
