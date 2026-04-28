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
pub struct PoliticalOrientationFilter {
    legis_period: Option<String>,
    gender: Option<String>,
    party: Option<String>,
    is_desc: bool,
    orientation_type: String, // "left" or "liberal"
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PoliticalOrientationBase {
    delegate_name: String,
    delegate_party: String,
    delegate_gender: String,
    orientation_score: f64,
    total_votes: i64,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PoliticalOrientationForDelegate {
    delegate_name: String,
    delegate_party: String,
    orientation_score: f64,
    total_votes: i64,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PoliticalOrientationByCategory {
    category: String,
    average_orientation: f64,
    total_votes: i64,
    delegate_count: i64,
}

pub struct PoliticalOrientationService;

impl PoliticalOrientationService {
    pub async fn get_base_data(
        pg: &sqlx::PgPool,
        filter: &PoliticalOrientationFilter,
    ) -> Result<Vec<PoliticalOrientationBase>, StatisticsResponse> {
        let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
        let filter_arg1 = filter.party.with_sql_column("m.party");
        let filter_arg2 = filter.gender.with_sql_column("ds.gender");
        let filter_arg3 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
        let filters = [filter_arg, filter_arg1, filter_arg2, filter_arg3];

        let filter_str = build_filter(&filters);

        let orientation_column = match filter.orientation_type.as_str() {
            "left" => "dv.is_left_vote",
            "liberal" => "dv.is_liberal_vote",
            _ => "dv.is_left_vote", // default to left
        };

        let query = format!(
            "
        SELECT 
            d.name AS delegate_name,
            COALESCE(m.party, 'Regierungsmitglied') AS delegate_party,
            d.gender AS delegate_gender,
            AVG(CASE WHEN {} = true THEN 1.0 ELSE 0.0 END) AS orientation_score,
            COUNT(dv.id) AS total_votes
        FROM 
            delegate_votes dv
        JOIN 
            delegates d ON dv.delegate_id = d.id
        LEFT JOIN mandates m ON m.delegate_id = d.id
        LEFT JOIN plenar_infos pf ON pf.id = dv.plenar_id
        WHERE 
            {} IS NOT NULL
            AND {filter_str}
        GROUP BY 
            d.id, d.name, m.party, d.gender
        ORDER BY 
            orientation_score DESC;
        ",
            orientation_column, orientation_column
        );

        let mut filtered_query = sqlx::query_as::<Postgres, PoliticalOrientationBase>(&query);
        filtered_query = bind_values(filtered_query, &filters);

        filtered_query
            .fetch_all(pg)
            .await
            .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
    }

    pub async fn per_delegate(
        pg: &sqlx::PgPool,
        filter: &PoliticalOrientationFilter,
    ) -> Result<Vec<PoliticalOrientationForDelegate>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        
        let mut results: Vec<PoliticalOrientationForDelegate> = base_data
            .into_iter()
            .map(|item| PoliticalOrientationForDelegate {
                delegate_name: item.delegate_name,
                delegate_party: item.delegate_party,
                orientation_score: item.orientation_score,
                total_votes: item.total_votes,
            })
            .collect();

        results.sort_by(|a, b| {
            b.orientation_score
                .partial_cmp(&a.orientation_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_party(
        pg: &sqlx::PgPool,
        filter: &PoliticalOrientationFilter,
    ) -> Result<Vec<PoliticalOrientationByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        
        let mut party_map: std::collections::HashMap<String, (Vec<f64>, i64, i64)> = std::collections::HashMap::new();
        
        for item in base_data {
            let entry = party_map.entry(item.delegate_party.clone()).or_insert((Vec::new(), 0, 0));
            entry.0.push(item.orientation_score);
            entry.1 += item.total_votes;
            entry.2 += 1; // delegate count
        }

        let mut results: Vec<PoliticalOrientationByCategory> = party_map
            .into_iter()
            .map(|(party, (scores, total_votes, delegate_count))| {
                let average_orientation = if !scores.is_empty() {
                    scores.iter().sum::<f64>() / scores.len() as f64
                } else {
                    0.0
                };
                
                PoliticalOrientationByCategory {
                    category: party,
                    average_orientation,
                    total_votes,
                    delegate_count,
                }
            })
            .collect();

        results.sort_by(|a, b| {
            b.average_orientation
                .partial_cmp(&a.average_orientation)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_gender(
        pg: &sqlx::PgPool,
        filter: &PoliticalOrientationFilter,
    ) -> Result<Vec<PoliticalOrientationByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        
        let mut gender_map: std::collections::HashMap<String, (Vec<f64>, i64, i64)> = std::collections::HashMap::new();
        
        for item in base_data {
            let entry = gender_map.entry(item.delegate_gender.clone()).or_insert((Vec::new(), 0, 0));
            entry.0.push(item.orientation_score);
            entry.1 += item.total_votes;
            entry.2 += 1; // delegate count
        }

        let mut results: Vec<PoliticalOrientationByCategory> = gender_map
            .into_iter()
            .map(|(gender, (scores, total_votes, delegate_count))| {
                let average_orientation = if !scores.is_empty() {
                    scores.iter().sum::<f64>() / scores.len() as f64
                } else {
                    0.0
                };
                
                PoliticalOrientationByCategory {
                    category: gender,
                    average_orientation,
                    total_votes,
                    delegate_count,
                }
            })
            .collect();

        results.sort_by(|a, b| {
            b.average_orientation
                .partial_cmp(&a.average_orientation)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_legis(
        pg: &sqlx::PgPool,
        filter: &PoliticalOrientationFilter,
    ) -> Result<Vec<PoliticalOrientationByCategory>, StatisticsResponse> {
        let filter_arg = filter.legis_period.with_sql_column("pf.legislative_period");
        let filter_arg1 = filter.gender.with_sql_column("ds.gender");
        let filter_arg2 = filter.party.with_sql_column("m.party");
        let filter_arg3 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
        let filters = [filter_arg, filter_arg1, filter_arg2, filter_arg3];

        let filter_str = build_filter(&filters);

        let orientation_column = match filter.orientation_type.as_str() {
            "left" => "dv.is_left_vote",
            "liberal" => "dv.is_liberal_vote",
            _ => "dv.is_left_vote",
        };

        let query = format!(
            "
        SELECT 
            pf.legislative_period AS category,
            AVG(CASE WHEN {} = true THEN 1.0 ELSE 0.0 END) AS average_orientation,
            COUNT(dv.id) AS total_votes,
            COUNT(DISTINCT d.id) AS delegate_count
        FROM 
            delegate_votes dv
        JOIN 
            delegates d ON dv.delegate_id = d.id
        LEFT JOIN mandates m ON m.delegate_id = d.id
        JOIN plenar_infos pf ON pf.id = dv.plenar_id
        WHERE 
            {} IS NOT NULL
            AND {filter_str}
        GROUP BY 
            pf.legislative_period
        ORDER BY 
            average_orientation DESC;
        ",
            orientation_column, orientation_column
        );

        let mut filtered_query = sqlx::query_as::<Postgres, PoliticalOrientationByCategory>(&query);
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
        filter: &PoliticalOrientationFilter,
    ) -> Result<Vec<PoliticalOrientationByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        
        let mut results: Vec<PoliticalOrientationByCategory> = vec![
            PoliticalOrientationByCategory {
                category: "18-30".to_string(),
                average_orientation: 0.0,
                total_votes: 0,
                delegate_count: 0,
            },
            PoliticalOrientationByCategory {
                category: "31-40".to_string(),
                average_orientation: 0.0,
                total_votes: 0,
                delegate_count: 0,
            },
            PoliticalOrientationByCategory {
                category: "41-50".to_string(),
                average_orientation: 0.0,
                total_votes: 0,
                delegate_count: 0,
            },
            PoliticalOrientationByCategory {
                category: "51-60".to_string(),
                average_orientation: 0.0,
                total_votes: 0,
                delegate_count: 0,
            },
            PoliticalOrientationByCategory {
                category: "60+".to_string(),
                average_orientation: 0.0,
                total_votes: 0,
                delegate_count: 0,
            },
        ];

        let scores: Vec<f64> = base_data.iter().map(|item| item.orientation_score).collect();
        let total_votes: i64 = base_data.iter().map(|item| item.total_votes).sum();
        let delegate_count: i64 = base_data.len() as i64;

        let average_orientation = if !scores.is_empty() {
            scores.iter().sum::<f64>() / scores.len() as f64
        } else {
            0.0
        };

        results.push(PoliticalOrientationByCategory {
            category: "Unknown".to_string(),
            average_orientation,
            total_votes,
            delegate_count,
        });

        results.sort_by(|a, b| {
            b.average_orientation
                .partial_cmp(&a.average_orientation)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }
}

// Votes Together endpoint
#[derive(ToSchema, Default, Debug, Clone, Serialize, Deserialize)]
pub struct VotesTogetherFilter {
    legis_period: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct VotesTogether {
    party_1: String,
    party_2: String,
    same_votes: i64,
}

pub async fn votes_together(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<VotesTogetherFilter>>,
) -> Result<Json<Vec<VotesTogether>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    println!("🔍 STATISTICS ENDPOINT: votes_together called with filter: {:?}", filter);

    let filter_arg = filter.legis_period.with_sql_column("gp");
    let filters = [filter_arg];

    let desc = if filter.is_desc { "DESC" } else { "ASC" };

    let filter_str = build_filter(&filters);

    let query = format!(
        "
        
WITH paired_votes AS (
    SELECT 
        v1.party AS party_1,
        v2.party AS party_2,
        COUNT(*) AS same_votes
    FROM 
        votes v1
    JOIN 
        votes v2
    ON 
        v1.legislative_initiatives_id = v2.legislative_initiatives_id
        AND v1.infavor = v2.infavor
        AND v1.party < v2.party  
    JOIN 
        legislative_initiatives li
    ON 
        v1.legislative_initiatives_id = li.id
    WHERE 
        {filter_str}
	 
    GROUP BY 
        v1.party, v2.party
)
SELECT 
    party_1,
    party_2,
    same_votes
FROM 
    paired_votes
ORDER BY 
    same_votes {desc};
        "
    );

    let mut filtered_query = sqlx::query_as::<Postgres, VotesTogether>(&query);
    filtered_query = bind_values(filtered_query, &filters);

    let results = filtered_query
        .fetch_all(&pg)
        .await
        .map(Json)
        .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))?;
    println!("✅ STATISTICS ENDPOINT: votes_together returning {} results", results.len());
    Ok(results)
}

pub async fn is_left_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationForDelegate>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "left".to_string();
    println!("🔍 STATISTICS ENDPOINT: is_left_per_delegate called with filter: {:?}", filter);
    let results = PoliticalOrientationService::per_delegate(&pg, &filter).await?;
    println!("✅ STATISTICS ENDPOINT: is_left_per_delegate returning {} results", results.len());
    Ok(Json(results))
}

pub async fn is_left_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "left".to_string();
    println!("🔍 STATISTICS ENDPOINT: is_left_per_party called with filter: {:?}", filter);
    let results = PoliticalOrientationService::per_party(&pg, &filter).await?;
    println!("✅ STATISTICS ENDPOINT: is_left_per_party returning {} results", results.len());
    Ok(Json(results))
}

pub async fn is_left_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "left".to_string();
    println!("🔍 STATISTICS ENDPOINT: is_left_per_gender called with filter: {:?}", filter);
    let results = PoliticalOrientationService::per_gender(&pg, &filter).await?;
    println!("✅ STATISTICS ENDPOINT: is_left_per_gender returning {} results", results.len());
    Ok(Json(results))
}

pub async fn is_left_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "left".to_string();
    println!("🔍 STATISTICS ENDPOINT: is_left_per_legis called with filter: {:?}", filter);
    let results = PoliticalOrientationService::per_legis(&pg, &filter).await?;
    println!("✅ STATISTICS ENDPOINT: is_left_per_legis returning {} results", results.len());
    Ok(Json(results))
}

pub async fn is_left_per_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "left".to_string();
    println!("🔍 STATISTICS ENDPOINT: is_left_per_age called with filter: {:?}", filter);
    let results = PoliticalOrientationService::per_age(&pg, &filter).await?;
    println!("✅ STATISTICS ENDPOINT: is_left_per_age returning {} results", results.len());
    Ok(Json(results))
}

// Legacy endpoint functions for backward compatibility - Is Liberal
pub async fn is_liberal_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationForDelegate>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "liberal".to_string();
    println!("🔍 STATISTICS ENDPOINT: is_liberal_per_delegate called with filter: {:?}", filter);
    let results = PoliticalOrientationService::per_delegate(&pg, &filter).await?;
    println!("✅ STATISTICS ENDPOINT: is_liberal_per_delegate returning {} results", results.len());
    Ok(Json(results))
}

pub async fn is_liberal_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "liberal".to_string();
    println!("🔍 STATISTICS ENDPOINT: is_liberal_per_party called with filter: {:?}", filter);
    let results = PoliticalOrientationService::per_party(&pg, &filter).await?;
    println!("✅ STATISTICS ENDPOINT: is_liberal_per_party returning {} results", results.len());
    Ok(Json(results))
}

pub async fn is_liberal_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "liberal".to_string();
    println!("🔍 STATISTICS ENDPOINT: is_liberal_per_gender called with filter: {:?}", filter);
    let results = PoliticalOrientationService::per_gender(&pg, &filter).await?;
    println!("✅ STATISTICS ENDPOINT: is_liberal_per_gender returning {} results", results.len());
    Ok(Json(results))
}

pub async fn is_liberal_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "liberal".to_string();
    println!("🔍 STATISTICS ENDPOINT: is_liberal_per_legis called with filter: {:?}", filter);
    let results = PoliticalOrientationService::per_legis(&pg, &filter).await?;
    println!("✅ STATISTICS ENDPOINT: is_liberal_per_legis returning {} results", results.len());
    Ok(Json(results))
}

pub async fn is_liberal_per_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "liberal".to_string();
    println!("🔍 STATISTICS ENDPOINT: is_liberal_per_age called with filter: {:?}", filter);
    let results = PoliticalOrientationService::per_age(&pg, &filter).await?;
    println!("✅ STATISTICS ENDPOINT: is_liberal_per_age returning {} results", results.len());
    Ok(Json(results))
}
