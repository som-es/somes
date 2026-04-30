use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres};
use utoipa::ToSchema;

use crate::{
    routes::statistics::routes::error::StatisticsResponse,
    routes::statistics::routes::filtering::{bind_values, build_filter, IntoFilterArgument, Manual},
    PgPoolConnection,
};

#[derive(ToSchema, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityFilter {
    legis_period: Option<String>,
    gender: Option<String>,
    party: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ComplexityBase {
    delegate_name: String,
    delegate_party: String,
    delegate_gender: String,
    complexity_score: f64,
    total_proposals: i64,
    legislative_period: Option<String>,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ComplexityForDelegate {
    delegate_name: String,
    delegate_party: String,
    complexity_score: f64,
    total_proposals: i64,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ComplexityByCategory {
    category: String,
    average_complexity: f64,
    total_proposals: i64,
    delegate_count: i64,
}

pub struct ComplexityService;

impl ComplexityService {
    pub async fn get_base_data(
        pg: &sqlx::PgPool,
        filter: &ComplexityFilter,
    ) -> Result<Vec<ComplexityBase>, StatisticsResponse> {
        let filter_arg1 = filter.party.with_sql_column("m.party");
        let filter_arg2 = filter.gender.with_sql_column("d.gender");
        let filter_arg3 = filter.legis_period.with_sql_column("p.gp");
        let filter_arg4 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
        let filters = [filter_arg1, filter_arg2, filter_arg3, filter_arg4];

        let filter_str = build_filter(&filters);

        let query = format!(
            "
        SELECT DISTINCT ON (d.id)
            d.name AS delegate_name,
            COALESCE(m.party, 'Regierungsmitglied') AS delegate_party,
            COALESCE(d.gender, '') AS delegate_gender,
            AVG(
                CASE 
                    WHEN p.ityp = 'J' THEN 1.0
                    WHEN p.ityp = 'AA' THEN 1.2
                    WHEN p.ityp = 'A' THEN 1.2
                    WHEN p.ityp = 'UEA' THEN 1.15
                    WHEN p.ityp = 'I' THEN 1.3
                    ELSE 1.0
                END
            )::FLOAT8 AS complexity_score,
            COUNT(p.id) AS total_proposals,
            p.gp AS legislative_period
        FROM 
            proposals p
        JOIN 
            proposal_delegates pd ON p.id = pd.proposal_id
        JOIN 
            delegates d ON pd.delegate_id = d.id
        LEFT JOIN mandates m ON m.delegate_id = d.id
        WHERE 
            pd.is_receiver = false
            AND {filter_str}
        GROUP BY 
            d.id, d.name, d.gender, m.party, p.gp
        ORDER BY 
            d.id, complexity_score DESC;
        "
        );

        let mut filtered_query = sqlx::query_as::<Postgres, ComplexityBase>(&query);
        filtered_query = bind_values(filtered_query, &filters);

        filtered_query
            .fetch_all(pg)
            .await
            .map_err(|e| {
                println!("Error fetching complexity data: {}", e);
                StatisticsResponse::DbSelectFailure(Some(e))
            })
    }

    pub async fn per_delegate(
        pg: &sqlx::PgPool,
        filter: &ComplexityFilter,
    ) -> Result<Vec<ComplexityForDelegate>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        let mut results: Vec<ComplexityForDelegate> = base_data
            .into_iter()
            .map(|item| ComplexityForDelegate {
                delegate_name: item.delegate_name,
                delegate_party: item.delegate_party,
                complexity_score: item.complexity_score,
                total_proposals: item.total_proposals,
            })
            .collect();

        results.sort_by(|a, b| {
            b.complexity_score
                .partial_cmp(&a.complexity_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_party(
        pg: &sqlx::PgPool,
        filter: &ComplexityFilter,
    ) -> Result<Vec<ComplexityByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        
        let mut party_map: std::collections::HashMap<String, (Vec<f64>, i64, i64)> = std::collections::HashMap::new();
        
        for item in base_data {
            let entry = party_map.entry(item.delegate_party.clone()).or_insert((Vec::new(), 0, 0));
            entry.0.push(item.complexity_score);
            entry.1 += item.total_proposals;
            entry.2 += 1; // delegate count
        }

        let mut results: Vec<ComplexityByCategory> = party_map
            .into_iter()
            .map(|(party, (scores, total_proposals, delegate_count))| {
                let average_complexity = if !scores.is_empty() {
                    scores.iter().sum::<f64>() / scores.len() as f64
                } else {
                    0.0
                };
                
                ComplexityByCategory {
                    category: party,
                    average_complexity,
                    total_proposals,
                    delegate_count,
                }
            })
            .collect();

        results.sort_by(|a, b| {
            b.average_complexity
                .partial_cmp(&a.average_complexity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_gender(
        pg: &sqlx::PgPool,
        filter: &ComplexityFilter,
    ) -> Result<Vec<ComplexityByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        
        let mut gender_map: std::collections::HashMap<String, (Vec<f64>, i64, i64)> = std::collections::HashMap::new();
        
        for item in base_data {
            let entry = gender_map.entry(item.delegate_gender.clone()).or_insert((Vec::new(), 0, 0));
            entry.0.push(item.complexity_score);
            entry.1 += item.total_proposals;
            entry.2 += 1; // delegate count
        }

        let mut results: Vec<ComplexityByCategory> = gender_map
            .into_iter()
            .map(|(gender, (scores, total_proposals, delegate_count))| {
                let average_complexity = if !scores.is_empty() {
                    scores.iter().sum::<f64>() / scores.len() as f64
                } else {
                    0.0
                };
                
                ComplexityByCategory {
                    category: gender,
                    average_complexity,
                    total_proposals,
                    delegate_count,
                }
            })
            .collect();

        results.sort_by(|a, b| {
            b.average_complexity
                .partial_cmp(&a.average_complexity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_legis(
        pg: &sqlx::PgPool,
        filter: &ComplexityFilter,
    ) -> Result<Vec<ComplexityByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        
        // Group by legislative_period
        let mut period_map: std::collections::HashMap<String, (Vec<f64>, i64, i64)> = std::collections::HashMap::new();
        
        for item in base_data {
            if let Some(period) = item.legislative_period {
                let entry = period_map.entry(period).or_insert((Vec::new(), 0, 0));
                entry.0.push(item.complexity_score);
                entry.1 += item.total_proposals;
                entry.2 += 1; // delegate count
            }
        }
        
        let mut results: Vec<ComplexityByCategory> = period_map
            .into_iter()
            .map(|(period, (scores, total_proposals, delegate_count))| {
                let average_complexity = if !scores.is_empty() {
                    scores.iter().sum::<f64>() / scores.len() as f64
                } else {
                    0.0
                };
                
                ComplexityByCategory {
                    category: period,
                    average_complexity,
                    total_proposals,
                    delegate_count,
                }
            })
            .collect();
        
        // Sort based on filter parameters
        results.sort_by(|a, b| {
            b.average_complexity
                .partial_cmp(&a.average_complexity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_age(
        pg: &sqlx::PgPool,
        filter: &ComplexityFilter,
    ) -> Result<Vec<ComplexityByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        
        // For age grouping, we'd need to calculate ages from birth dates
        // This is a simplified version - you might need to adjust based on your actual age calculation logic
        let mut results: Vec<ComplexityByCategory> = vec![
            ComplexityByCategory {
                category: "18-30".to_string(),
                average_complexity: 0.0,
                total_proposals: 0,
                delegate_count: 0,
            },
            ComplexityByCategory {
                category: "31-40".to_string(),
                average_complexity: 0.0,
                total_proposals: 0,
                delegate_count: 0,
            },
            ComplexityByCategory {
                category: "41-50".to_string(),
                average_complexity: 0.0,
                total_proposals: 0,
                delegate_count: 0,
            },
            ComplexityByCategory {
                category: "51-60".to_string(),
                average_complexity: 0.0,
                total_proposals: 0,
                delegate_count: 0,
            },
            ComplexityByCategory {
                category: "60+".to_string(),
                average_complexity: 0.0,
                total_proposals: 0,
                delegate_count: 0,
            },
        ];

        // Note: You'll need to implement actual age calculation logic here
        // This is a placeholder that aggregates all data into "Unknown" category
        let scores: Vec<f64> = base_data.iter().map(|item| item.complexity_score).collect();
        let total_proposals: i64 = base_data.iter().map(|item| item.total_proposals).sum();
        let delegate_count: i64 = base_data.len() as i64;

        let average_complexity = if !scores.is_empty() {
            scores.iter().sum::<f64>() / scores.len() as f64
        } else {
            0.0
        };

        results.push(ComplexityByCategory {
            category: "Unknown".to_string(),
            average_complexity,
            total_proposals,
            delegate_count,
        });

        results.sort_by(|a, b| {
            b.average_complexity
                .partial_cmp(&a.average_complexity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }
}

pub async fn complexity_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ComplexityFilter>>,
) -> Result<Json<Vec<ComplexityForDelegate>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ComplexityService::per_delegate(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn complexity_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ComplexityFilter>>,
) -> Result<Json<Vec<ComplexityByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ComplexityService::per_party(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn complexity_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ComplexityFilter>>,
) -> Result<Json<Vec<ComplexityByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ComplexityService::per_gender(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn complexity_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ComplexityFilter>>,
) -> Result<Json<Vec<ComplexityByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ComplexityService::per_legis(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn complexity_at_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<ComplexityFilter>>,
) -> Result<Json<Vec<ComplexityByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = ComplexityService::per_age(&pg, &filter).await?;
    Ok(Json(results))
}
