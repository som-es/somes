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
pub struct AgeFilter {
    legis_period: Option<String>,
    gender: Option<String>,
    party: Option<String>,
    is_desc: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AgeBase {
    delegate_name: String,
    delegate_party: String,
    delegate_gender: String,
    age: i32,
    birthdate: Option<chrono::NaiveDate>,
    legislative_period: Option<String>,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AgeForDelegate {
    delegate_name: String,
    delegate_party: String,
    age: i32,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AgeByCategory {
    category: String,
    average_age: f64,
    delegate_count: i64,
    min_age: i32,
    max_age: i32,
}

pub struct AgeService;

impl AgeService {
    pub async fn get_base_data(
        pg: &sqlx::PgPool,
        filter: &AgeFilter,
    ) -> Result<Vec<AgeBase>, StatisticsResponse> {
        let filter_arg = filter.legis_period.with_sql_column("lp.legislative_period");
        let filter_arg1 = filter.party.with_sql_column("m.party");
        let filter_arg2 = filter.gender.with_sql_column("d.gender");
        let filter_arg3 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
        let filters = [filter_arg, filter_arg1, filter_arg2, filter_arg3];

        let filter_str = build_filter(&filters);

        let query = format!(
            "
        WITH legislative_period_dates AS (
            SELECT 
                legislative_period, 
                MIN(raw_data_created_at) AS start_date, 
                MAX(raw_data_created_at) AS end_date
            FROM 
                plenar_infos
            GROUP BY 
                legislative_period
        )
        SELECT DISTINCT ON (d.id, lp.legislative_period, m.party)
            d.name AS delegate_name,
            COALESCE(m.party, 'Regierungsmitglied') AS delegate_party,
            d.gender AS delegate_gender,
            EXTRACT(YEAR FROM AGE(GREATEST(m.start_date, lp.start_date::date), d.birthdate))::INT AS age,
            d.birthdate,
            lp.legislative_period
        FROM 
            delegates d
        CROSS JOIN legislative_period_dates lp
        JOIN LATERAL (
            SELECT *
            FROM mandates m
            WHERE
                m.delegate_id = d.id
                AND (m.is_nr OR m.is_gov_official)
                AND (m.start_date IS NULL OR m.start_date <= lp.end_date::date)
                AND (m.end_date IS NULL OR m.end_date >= lp.start_date::date)
            ORDER BY
                COALESCE(m.end_date, lp.end_date::date) DESC,
                m.start_date DESC
            LIMIT 1
        ) m ON true
        WHERE 
            d.birthdate IS NOT NULL
            AND {filter_str}
        ORDER BY 
            d.id, lp.legislative_period, m.party, age DESC;
        "
        );

        let mut filtered_query = sqlx::query_as::<Postgres, AgeBase>(&query);
        filtered_query = bind_values(filtered_query, &filters);

        filtered_query
            .fetch_all(pg)
            .await
            .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
    }

    pub async fn per_delegate(
        pg: &sqlx::PgPool,
        filter: &AgeFilter,
    ) -> Result<Vec<AgeForDelegate>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        let mut results: Vec<AgeForDelegate> = base_data
            .into_iter()
            .map(|item| AgeForDelegate {
                delegate_name: item.delegate_name,
                delegate_party: item.delegate_party,
                age: item.age,
            })
            .collect();

        results.sort_by(|a, b| b.age.cmp(&a.age));

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_party(
        pg: &sqlx::PgPool,
        filter: &AgeFilter,
    ) -> Result<Vec<AgeByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;

        let mut party_map: std::collections::HashMap<String, Vec<i32>> =
            std::collections::HashMap::new();

        for item in base_data {
            party_map
                .entry(item.delegate_party.clone())
                .or_insert_with(Vec::new)
                .push(item.age);
        }

        let mut results: Vec<AgeByCategory> = party_map
            .into_iter()
            .map(|(party, ages)| {
                let delegate_count = ages.len() as i64;
                let average_age = ages.iter().sum::<i32>() as f64 / delegate_count as f64;
                let min_age = *ages.iter().min().unwrap_or(&0);
                let max_age = *ages.iter().max().unwrap_or(&0);

                AgeByCategory {
                    category: party,
                    average_age,
                    delegate_count,
                    min_age,
                    max_age,
                }
            })
            .collect();

        results.sort_by(|a, b| {
            b.average_age
                .partial_cmp(&a.average_age)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_gender(
        pg: &sqlx::PgPool,
        filter: &AgeFilter,
    ) -> Result<Vec<AgeByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;

        let mut gender_map: std::collections::HashMap<String, Vec<i32>> =
            std::collections::HashMap::new();

        for item in base_data {
            gender_map
                .entry(item.delegate_gender.clone())
                .or_insert_with(Vec::new)
                .push(item.age);
        }

        let mut results: Vec<AgeByCategory> = gender_map
            .into_iter()
            .map(|(gender, ages)| {
                let delegate_count = ages.len() as i64;
                let average_age = ages.iter().sum::<i32>() as f64 / delegate_count as f64;
                let min_age = *ages.iter().min().unwrap_or(&0);
                let max_age = *ages.iter().max().unwrap_or(&0);

                AgeByCategory {
                    category: gender,
                    average_age,
                    delegate_count,
                    min_age,
                    max_age,
                }
            })
            .collect();

        results.sort_by(|a, b| {
            b.average_age
                .partial_cmp(&a.average_age)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_legis(
        pg: &sqlx::PgPool,
        filter: &AgeFilter,
    ) -> Result<Vec<AgeByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;

        let mut legis_map: std::collections::HashMap<String, Vec<i32>> =
            std::collections::HashMap::new();

        for item in base_data {
            if let Some(period) = item.legislative_period {
                legis_map
                    .entry(period)
                    .or_insert_with(Vec::new)
                    .push(item.age);
            }
        }

        let mut results: Vec<AgeByCategory> = legis_map
            .into_iter()
            .map(|(period, ages)| {
                let delegate_count = ages.len() as i64;
                let average_age = ages.iter().sum::<i32>() as f64 / delegate_count as f64;
                let min_age = *ages.iter().min().unwrap_or(&0);
                let max_age = *ages.iter().max().unwrap_or(&0);

                AgeByCategory {
                    category: period,
                    average_age,
                    delegate_count,
                    min_age,
                    max_age,
                }
            })
            .collect();

        results.sort_by(|a, b| {
            b.average_age
                .partial_cmp(&a.average_age)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_age(
        pg: &sqlx::PgPool,
        filter: &AgeFilter,
    ) -> Result<Vec<AgeByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;

        let mut age_map: std::collections::HashMap<String, Vec<i32>> =
            std::collections::HashMap::new();

        for item in base_data {
            let category = match item.age {
                age if age <= 30 => "18-30",
                age if age <= 40 => "31-40",
                age if age <= 50 => "41-50",
                age if age <= 60 => "51-60",
                _ => "60+",
            };
            age_map
                .entry(category.to_string())
                .or_insert_with(Vec::new)
                .push(item.age);
        }

        let mut results: Vec<AgeByCategory> = age_map
            .into_iter()
            .map(|(category, ages)| {
                let delegate_count = ages.len() as i64;
                let average_age = ages.iter().sum::<i32>() as f64 / delegate_count as f64;
                let min_age = *ages.iter().min().unwrap_or(&0);
                let max_age = *ages.iter().max().unwrap_or(&0);

                AgeByCategory {
                    category,
                    average_age,
                    delegate_count,
                    min_age,
                    max_age,
                }
            })
            .collect();

        results.sort_by(|a, b| {
            b.average_age
                .partial_cmp(&a.average_age)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }
}

// Legacy endpoint functions for backward compatibility
pub async fn age_of_delegates(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<AgeFilter>>,
) -> Result<Json<Vec<AgeForDelegate>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = AgeService::per_delegate(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn age_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<AgeFilter>>,
) -> Result<Json<Vec<AgeByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = AgeService::per_party(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn age_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<AgeFilter>>,
) -> Result<Json<Vec<AgeByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = AgeService::per_gender(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn age_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<AgeFilter>>,
) -> Result<Json<Vec<AgeByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = AgeService::per_legis(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn age_per_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<AgeFilter>>,
) -> Result<Json<Vec<AgeByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = AgeService::per_age(&pg, &filter).await?;
    Ok(Json(results))
}
