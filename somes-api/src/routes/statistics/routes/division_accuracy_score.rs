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
    delegate_gender: String,
    accuracy_score: f64,
    total_votes: i64,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DivisionAccuracyForDelegate {
    delegate_name: String,
    delegate_party: String,
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
    pub async fn get_base_data(
        pg: &sqlx::PgPool,
        filter: &DivisionAccuracyFilter,
    ) -> Result<Vec<DivisionAccuracyBase>, StatisticsResponse> {
        let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
        let filter_arg1 = filter.party.with_sql_column("m.party");
        let filter_arg2 = filter.gender.with_sql_column("d.gender");
        let filter_arg3 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
        let filters = [filter_arg, filter_arg1, filter_arg2, filter_arg3];

        let filter_str = build_filter(&filters);

        let query = format!(
            "
        SELECT 
            d.name AS delegate_name,
            COALESCE(m.party, 'Regierungsmitglied') AS delegate_party,
            d.gender AS delegate_gender,
            AVG(CASE WHEN dv.vote = dv.outcome THEN 1.0 ELSE 0.0 END) AS accuracy_score,
            COUNT(dv.id) AS total_votes
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
            d.id, d.name, m.party, d.gender
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

        let mut results: Vec<DivisionAccuracyForDelegate> = base_data
            .into_iter()
            .map(|item| DivisionAccuracyForDelegate {
                delegate_name: item.delegate_name,
                delegate_party: item.delegate_party,
                accuracy_score: item.accuracy_score,
                total_votes: item.total_votes,
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

        let mut party_map: std::collections::HashMap<String, (Vec<f64>, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry = party_map
                .entry(item.delegate_party.clone())
                .or_insert((Vec::new(), 0, 0));
            entry.0.push(item.accuracy_score);
            entry.1 += item.total_votes;
            entry.2 += 1; // delegate count
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

        results.sort_by(|a, b| {
            b.average_accuracy
                .partial_cmp(&a.average_accuracy)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_gender(
        pg: &sqlx::PgPool,
        filter: &DivisionAccuracyFilter,
    ) -> Result<Vec<DivisionAccuracyByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;

        let mut gender_map: std::collections::HashMap<String, (Vec<f64>, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry =
                gender_map
                    .entry(item.delegate_gender.clone())
                    .or_insert((Vec::new(), 0, 0));
            entry.0.push(item.accuracy_score);
            entry.1 += item.total_votes;
            entry.2 += 1; // delegate count
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

        results.sort_by(|a, b| {
            b.average_accuracy
                .partial_cmp(&a.average_accuracy)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_legis(
        pg: &sqlx::PgPool,
        filter: &DivisionAccuracyFilter,
    ) -> Result<Vec<DivisionAccuracyByCategory>, StatisticsResponse> {
        let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
        let filter_arg1 = filter.gender.with_sql_column("d.gender");
        let filter_arg2 = filter.party.with_sql_column("m.party");
        let filter_arg3 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
        let filters = [filter_arg, filter_arg1, filter_arg2, filter_arg3];

        let filter_str = build_filter(&filters);

        let query = format!(
            "
        SELECT 
            pf.legislative_period AS category,
            AVG(CASE WHEN dv.vote = dv.outcome THEN 1.0 ELSE 0.0 END) AS average_accuracy,
            COUNT(dv.id) AS total_votes,
            COUNT(DISTINCT d.id) AS delegate_count
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
            pf.legislative_period
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

        // For age grouping, we'd need to calculate ages from birth dates
        // This is a simplified version - you might need to adjust based on your actual age calculation logic
        let mut results: Vec<DivisionAccuracyByCategory> = vec![
            DivisionAccuracyByCategory {
                category: "18-30".to_string(),
                average_accuracy: 0.0,
                total_votes: 0,
                delegate_count: 0,
            },
            DivisionAccuracyByCategory {
                category: "31-40".to_string(),
                average_accuracy: 0.0,
                total_votes: 0,
                delegate_count: 0,
            },
            DivisionAccuracyByCategory {
                category: "41-50".to_string(),
                average_accuracy: 0.0,
                total_votes: 0,
                delegate_count: 0,
            },
            DivisionAccuracyByCategory {
                category: "51-60".to_string(),
                average_accuracy: 0.0,
                total_votes: 0,
                delegate_count: 0,
            },
            DivisionAccuracyByCategory {
                category: "60+".to_string(),
                average_accuracy: 0.0,
                total_votes: 0,
                delegate_count: 0,
            },
        ];

        // Note: You'll need to implement actual age calculation logic here
        // This is a placeholder that aggregates all data into "Unknown" category
        let scores: Vec<f64> = base_data.iter().map(|item| item.accuracy_score).collect();
        let total_votes: i64 = base_data.iter().map(|item| item.total_votes).sum();
        let delegate_count: i64 = base_data.len() as i64;

        let average_accuracy = if !scores.is_empty() {
            scores.iter().sum::<f64>() / scores.len() as f64
        } else {
            0.0
        };

        results.push(DivisionAccuracyByCategory {
            category: "Unknown".to_string(),
            average_accuracy,
            total_votes,
            delegate_count,
        });

        results.sort_by(|a, b| {
            b.average_accuracy
                .partial_cmp(&a.average_accuracy)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }
}

// Legacy endpoint functions for backward compatibility
pub async fn division_accuracy_score_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DivisionAccuracyFilter>>,
) -> Result<Json<Vec<DivisionAccuracyForDelegate>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    println!(
        "🔍 STATISTICS ENDPOINT: division_accuracy_score_per_delegate called with filter: {:?}",
        filter
    );
    let results = DivisionAccuracyService::per_delegate(&pg, &filter).await?;
    println!(
        "✅ STATISTICS ENDPOINT: division_accuracy_score_per_delegate returning {} results",
        results.len()
    );
    Ok(Json(results))
}

pub async fn division_accuracy_score_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DivisionAccuracyFilter>>,
) -> Result<Json<Vec<DivisionAccuracyByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    println!(
        "🔍 STATISTICS ENDPOINT: division_accuracy_score_per_party called with filter: {:?}",
        filter
    );
    let results = DivisionAccuracyService::per_party(&pg, &filter).await?;
    println!(
        "✅ STATISTICS ENDPOINT: division_accuracy_score_per_party returning {} results",
        results.len()
    );
    Ok(Json(results))
}

pub async fn division_accuracy_score_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DivisionAccuracyFilter>>,
) -> Result<Json<Vec<DivisionAccuracyByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    println!(
        "🔍 STATISTICS ENDPOINT: division_accuracy_score_per_gender called with filter: {:?}",
        filter
    );
    let results = DivisionAccuracyService::per_gender(&pg, &filter).await?;
    println!(
        "✅ STATISTICS ENDPOINT: division_accuracy_score_per_gender returning {} results",
        results.len()
    );
    Ok(Json(results))
}

pub async fn division_accuracy_score_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DivisionAccuracyFilter>>,
) -> Result<Json<Vec<DivisionAccuracyByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    println!(
        "🔍 STATISTICS ENDPOINT: division_accuracy_score_per_legis called with filter: {:?}",
        filter
    );
    let results = DivisionAccuracyService::per_legis(&pg, &filter).await?;
    println!(
        "✅ STATISTICS ENDPOINT: division_accuracy_score_per_legis returning {} results",
        results.len()
    );
    Ok(Json(results))
}

pub async fn division_accuracy_score_per_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<DivisionAccuracyFilter>>,
) -> Result<Json<Vec<DivisionAccuracyByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    println!(
        "🔍 STATISTICS ENDPOINT: division_accuracy_score_per_age called with filter: {:?}",
        filter
    );
    let results = DivisionAccuracyService::per_age(&pg, &filter).await?;
    println!(
        "✅ STATISTICS ENDPOINT: division_accuracy_score_per_age returning {} results",
        results.len()
    );
    Ok(Json(results))
}
