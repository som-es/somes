use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, prelude::FromRow};
use utoipa::ToSchema;

use crate::{
    PgPoolConnection,
    routes::statistics::routes::error::StatisticsResponse,
    routes::statistics::routes::filtering::{
        IntoFilterArgument, Manual, bind_values, build_filter,
    },
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
    total_scores: i64,
    latest_activity_date: Option<chrono::NaiveDate>,
    delegate_age_bucket: String,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DivisionAccuracyForDelegate {
    delegate_name: String,
    delegate_party: String,
    delegate_filter_party: String,
    accuracy_score: f64,
    total_scores: i64,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DivisionAccuracyByCategory {
    category: String,
    average_accuracy: f64,
    total_scores: i64,
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
            entry.1 += item.total_scores;
            entry.2 += 1;
        }

        let mut results: Vec<DivisionAccuracyByCategory> = party_map
            .into_iter()
            .map(|(party, (scores, total_scores, delegate_count))| {
                let average_accuracy = if !scores.is_empty() {
                    scores.iter().sum::<f64>() / scores.len() as f64
                } else {
                    0.0
                };

                DivisionAccuracyByCategory {
                    category: party,
                    average_accuracy,
                    total_scores,
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
            let entry = gender_map
                .entry(
                    item.delegate_gender
                        .clone()
                        .unwrap_or_else(|| "Unknown".into()),
                )
                .or_insert((Vec::new(), 0, 0));
            entry.0.push(item.accuracy_score);
            entry.1 += item.total_scores;
            entry.2 += 1;
        }

        let mut results: Vec<DivisionAccuracyByCategory> = gender_map
            .into_iter()
            .map(|(gender, (scores, total_scores, delegate_count))| {
                let average_accuracy = if !scores.is_empty() {
                    scores.iter().sum::<f64>() / scores.len() as f64
                } else {
                    0.0
                };

                DivisionAccuracyByCategory {
                    category: gender,
                    average_accuracy,
                    total_scores,
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
            entry.1 += item.total_scores;
            entry.2 += 1;
        }

        let mut results: Vec<DivisionAccuracyByCategory> = age_map
            .into_iter()
            .map(|(category, (scores, total_scores, delegate_count))| {
                let average_accuracy = if !scores.is_empty() {
                    scores.iter().sum::<f64>() / scores.len() as f64
                } else {
                    0.0
                };

                DivisionAccuracyByCategory {
                    category,
                    average_accuracy,
                    total_scores,
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
        SELECT DISTINCT ON (d.id)
            d.name AS delegate_name,
            COALESCE(m.party, d.party, 'Regierungsmitglied') AS delegate_party,
            COALESCE(m.party, 'Regierungsmitglied') AS delegate_filter_party,
            d.gender AS delegate_gender,
            dis.score::float8 AS accuracy_score,
            1::bigint AS total_scores,
            pf.raw_data_created_at::date AS latest_activity_date,
            CASE
                WHEN d.birthdate IS NULL THEN 'Unbekannt'
                WHEN EXTRACT(YEAR FROM AGE(pf.raw_data_created_at::date, d.birthdate)) <= 30 THEN '18-30'
                WHEN EXTRACT(YEAR FROM AGE(pf.raw_data_created_at::date, d.birthdate)) <= 40 THEN '31-40'
                WHEN EXTRACT(YEAR FROM AGE(pf.raw_data_created_at::date, d.birthdate)) <= 50 THEN '41-50'
                WHEN EXTRACT(YEAR FROM AGE(pf.raw_data_created_at::date, d.birthdate)) <= 60 THEN '51-60'
                ELSE '60+'
            END AS delegate_age_bucket
        FROM delegates d
        JOIN LATERAL (
            SELECT score FROM division_interest_score
            WHERE delegate_id = d.id
            ORDER BY timestamp DESC NULLS LAST, id DESC LIMIT 1
        ) dis ON true
        JOIN plenar_speeches ps ON ps.delegate_id = d.id
        JOIN debates db ON db.id = ps.debate_id
        JOIN plenar_infos pf ON pf.id = db.plenar_id
        JOIN mandates m ON m.delegate_id = d.id
            AND (m.start_date IS NULL OR m.start_date <= pf.raw_data_created_at::date)
            AND (m.end_date IS NULL OR m.end_date >= pf.raw_data_created_at::date)
        WHERE {filter_str}
        ORDER BY d.id, pf.raw_data_created_at DESC NULLS LAST,
            m.start_date DESC NULLS LAST, m.id DESC;
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

        // Base data contains one latest qualifying affiliation per delegate.
        let mut results: Vec<DivisionAccuracyForDelegate> = base_data
            .into_iter()
            .map(|item| DivisionAccuracyForDelegate {
                delegate_name: item.delegate_name,
                delegate_party: item.delegate_party,
                delegate_filter_party: item.delegate_filter_party,
                accuracy_score: item.accuracy_score,
                total_scores: item.total_scores,
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
            SELECT DISTINCT
                pf.legislative_period AS category,
                d.id AS delegate_id,
                dis.score::float8 AS accuracy_score
            FROM delegates d
            JOIN LATERAL (
                SELECT score FROM division_interest_score
                WHERE delegate_id = d.id
                ORDER BY timestamp DESC NULLS LAST, id DESC LIMIT 1
            ) dis ON true
            JOIN plenar_speeches ps ON ps.delegate_id = d.id
            JOIN debates db ON db.id = ps.debate_id
            JOIN plenar_infos pf ON pf.id = db.plenar_id
            JOIN mandates m ON m.delegate_id = d.id
                AND (m.start_date IS NULL OR m.start_date <= pf.raw_data_created_at::date)
                AND (m.end_date IS NULL OR m.end_date >= pf.raw_data_created_at::date)
            WHERE pf.legislative_period IS NOT NULL AND {filter_str}
        )
        SELECT
            category,
            AVG(accuracy_score)::float8 AS average_accuracy,
            COUNT(*)::bigint AS total_scores,
            COUNT(*)::bigint AS delegate_count
        FROM delegate_period_accuracy
        GROUP BY category
        ORDER BY average_accuracy DESC;
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

// Public endpoints; the historical misspelling is registered as a router alias.
#[utoipa::path(
    post,
    path = "/division_accuracy_score_per_delegate",
    tag = "statistics",
    request_body(content = DivisionAccuracyFilter, content_type = "application/json"),
    responses(
        (status = 200, description = "Division accuracy score per delegate", body = [DivisionAccuracyForDelegate]),
    )
)]
pub async fn division_accuracy_score_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DivisionAccuracyFilter>>,
) -> Result<Json<Vec<DivisionAccuracyForDelegate>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = DivisionAccuracyService::per_delegate(&pg, &filter).await?;
    Ok(Json(results))
}

#[utoipa::path(
    post,
    path = "/division_accuracy_score_per_party",
    tag = "statistics",
    request_body(content = DivisionAccuracyFilter, content_type = "application/json"),
    responses(
        (status = 200, description = "Division accuracy score per party", body = [DivisionAccuracyByCategory]),
    )
)]
pub async fn division_accuracy_score_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DivisionAccuracyFilter>>,
) -> Result<Json<Vec<DivisionAccuracyByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = DivisionAccuracyService::per_party(&pg, &filter).await?;
    Ok(Json(results))
}

#[utoipa::path(
    post,
    path = "/division_accuracy_score_per_gender",
    tag = "statistics",
    request_body(content = DivisionAccuracyFilter, content_type = "application/json"),
    responses(
        (status = 200, description = "Division accuracy score per gender", body = [DivisionAccuracyByCategory]),
    )
)]
pub async fn division_accuracy_score_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DivisionAccuracyFilter>>,
) -> Result<Json<Vec<DivisionAccuracyByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = DivisionAccuracyService::per_gender(&pg, &filter).await?;
    Ok(Json(results))
}

#[utoipa::path(
    post,
    path = "/division_accuracy_score_per_legis",
    tag = "statistics",
    request_body(content = DivisionAccuracyFilter, content_type = "application/json"),
    responses(
        (status = 200, description = "Division accuracy score per legis", body = [DivisionAccuracyByCategory]),
    )
)]
pub async fn division_accuracy_score_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DivisionAccuracyFilter>>,
) -> Result<Json<Vec<DivisionAccuracyByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = DivisionAccuracyService::per_legis(&pg, &filter).await?;
    Ok(Json(results))
}

#[utoipa::path(
    post,
    path = "/division_accuracy_score_per_age",
    tag = "statistics",
    request_body(content = DivisionAccuracyFilter, content_type = "application/json"),
    responses(
        (status = 200, description = "Division accuracy score per age", body = [DivisionAccuracyByCategory]),
    )
)]
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
