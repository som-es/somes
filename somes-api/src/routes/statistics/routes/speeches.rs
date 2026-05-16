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
pub struct SpeechFilter {
    #[serde(default)]
    legis_period: Option<String>,
    #[serde(default)]
    gender: Option<String>,
    #[serde(default)]
    party: Option<String>,
    #[serde(default)]
    is_desc: bool,
    #[serde(default)]
    speech_type: String, // "speechtime" or "total_speeches"
    #[serde(default)]
    normalized: bool,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SpeechBase {
    delegate_name: String,
    delegate_party: String,
    delegate_gender: String,
    total_speeches: i64,
    total_speech_time: i64, // in seconds
    average_speech_time: f64,
    legislative_period: Option<String>,
    delegate_age_bucket: String,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SpeechForDelegate {
    delegate_name: String,
    delegate_party: String,
    total_speeches: i64,
    total_speech_time: i64,
    average_speech_time: f64,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SpeechByCategory {
    category: String,
    total_speeches: i64,
    total_speech_time: i64,
    average_speech_time: f64,
    delegate_count: i64,
}

pub struct SpeechService;

impl SpeechService {
    pub async fn get_base_data(
        pg: &sqlx::PgPool,
        filter: &SpeechFilter,
    ) -> Result<Vec<SpeechBase>, StatisticsResponse> {
        let filter_arg0 = filter.legis_period.with_sql_column("pf.legislative_period");
        let filter_arg1 = filter.party.with_sql_column("m.party");
        let filter_arg2 = filter.gender.with_sql_column("d.gender");
        let filter_arg3 = Manual("(m.is_nr OR m.is_gov_official)").with_sql_column("");
        let filters = [filter_arg0, filter_arg1, filter_arg2, filter_arg3];

        let filter_str = build_filter(&filters);

        let query = format!(
            "
        WITH legislative_period_dates AS (
            SELECT
                legislative_period,
                MIN(raw_data_created_at) AS start_date
            FROM plenar_infos
            GROUP BY legislative_period
        )
        SELECT 
            d.name AS delegate_name,
            COALESCE(m.party, 'Regierungsmitglied') AS delegate_party,
            d.gender AS delegate_gender,
            COUNT(ps.id) AS total_speeches,
            COALESCE(SUM(ps.duration_in_seconds), 0) AS total_speech_time,
            COALESCE(AVG(ps.duration_in_seconds)::FLOAT8, 0) AS average_speech_time,
            pf.legislative_period,
            CASE
                WHEN d.birthdate IS NULL THEN 'Unbekannt'
                WHEN EXTRACT(YEAR FROM AGE(lp.start_date, d.birthdate)) <= 30 THEN '18-30'
                WHEN EXTRACT(YEAR FROM AGE(lp.start_date, d.birthdate)) <= 40 THEN '31-40'
                WHEN EXTRACT(YEAR FROM AGE(lp.start_date, d.birthdate)) <= 50 THEN '41-50'
                WHEN EXTRACT(YEAR FROM AGE(lp.start_date, d.birthdate)) <= 60 THEN '51-60'
                ELSE '60+'
            END AS delegate_age_bucket
        FROM 
            plenar_speeches ps
        JOIN 
            delegates d ON ps.delegate_id = d.id
        LEFT JOIN debates db ON ps.debate_id = db.id
        LEFT JOIN plenar_infos pf ON db.plenar_id = pf.id
        LEFT JOIN legislative_period_dates lp ON lp.legislative_period = pf.legislative_period
        LEFT JOIN mandates m ON m.delegate_id = d.id
            AND (m.start_date IS NULL OR m.start_date <= pf.raw_data_created_at::date)
            AND (m.end_date IS NULL OR m.end_date >= pf.raw_data_created_at::date)
        WHERE 
            ps.duration_in_seconds IS NOT NULL
            AND {filter_str}
        GROUP BY 
            d.id, d.name, m.party, d.gender, d.birthdate, pf.legislative_period, lp.start_date
        ORDER BY 
            total_speech_time DESC;
        "
        );

        let mut filtered_query = sqlx::query_as::<Postgres, SpeechBase>(&query);
        filtered_query = bind_values(filtered_query, &filters);

        filtered_query
            .fetch_all(pg)
            .await
            .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
    }

    pub async fn per_delegate(
        pg: &sqlx::PgPool,
        filter: &SpeechFilter,
    ) -> Result<Vec<SpeechForDelegate>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        let mut results: Vec<SpeechForDelegate> = base_data
            .into_iter()
            .map(|item| SpeechForDelegate {
                delegate_name: item.delegate_name,
                delegate_party: item.delegate_party,
                total_speeches: item.total_speeches,
                total_speech_time: item.total_speech_time,
                average_speech_time: item.average_speech_time,
            })
            .collect();

        // Sort based on speech type and normalized flag
        match filter.speech_type.as_str() {
            "speechtime" => {
                if filter.normalized {
                    results.sort_by(|a, b| {
                        b.average_speech_time
                            .partial_cmp(&a.average_speech_time)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                } else {
                    results.sort_by(|a, b| b.total_speech_time.cmp(&a.total_speech_time));
                }
            }
            "total_speeches" => {
                results.sort_by(|a, b| b.total_speeches.cmp(&a.total_speeches));
            }
            _ => {
                if filter.normalized {
                    results.sort_by(|a, b| {
                        b.average_speech_time
                            .partial_cmp(&a.average_speech_time)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                } else {
                    results.sort_by(|a, b| b.total_speech_time.cmp(&a.total_speech_time));
                }
            }
        }

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_party(
        pg: &sqlx::PgPool,
        filter: &SpeechFilter,
    ) -> Result<Vec<SpeechByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;

        let mut party_map: std::collections::HashMap<String, (i64, i64, Vec<f64>, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry =
                party_map
                    .entry(item.delegate_party.clone())
                    .or_insert((0, 0, Vec::new(), 0));
            entry.0 += item.total_speeches;
            entry.1 += item.total_speech_time;
            if item.average_speech_time > 0.0 {
                entry.2.push(item.average_speech_time);
            }
            entry.3 += 1; // delegate count
        }

        let mut results: Vec<SpeechByCategory> = party_map
            .into_iter()
            .map(
                |(party, (total_speeches, total_speech_time, avg_times, delegate_count))| {
                    let average_speech_time = if !avg_times.is_empty() {
                        avg_times.iter().sum::<f64>() / avg_times.len() as f64
                    } else {
                        0.0
                    };

                    SpeechByCategory {
                        category: party,
                        total_speeches,
                        total_speech_time,
                        average_speech_time,
                        delegate_count,
                    }
                },
            )
            .collect();

        // Sort based on speech type and normalized flag
        match filter.speech_type.as_str() {
            "speechtime" => {
                if filter.normalized {
                    results.sort_by(|a, b| {
                        b.average_speech_time
                            .partial_cmp(&a.average_speech_time)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                } else {
                    results.sort_by(|a, b| b.total_speech_time.cmp(&a.total_speech_time));
                }
            }
            "total_speeches" => {
                results.sort_by(|a, b| b.total_speeches.cmp(&a.total_speeches));
            }
            _ => {
                if filter.normalized {
                    results.sort_by(|a, b| {
                        b.average_speech_time
                            .partial_cmp(&a.average_speech_time)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                } else {
                    results.sort_by(|a, b| b.total_speech_time.cmp(&a.total_speech_time));
                }
            }
        }

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_gender(
        pg: &sqlx::PgPool,
        filter: &SpeechFilter,
    ) -> Result<Vec<SpeechByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;

        let mut gender_map: std::collections::HashMap<String, (i64, i64, Vec<f64>, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry =
                gender_map
                    .entry(item.delegate_gender.clone())
                    .or_insert((0, 0, Vec::new(), 0));
            entry.0 += item.total_speeches;
            entry.1 += item.total_speech_time;
            if item.average_speech_time > 0.0 {
                entry.2.push(item.average_speech_time);
            }
            entry.3 += 1; // delegate count
        }

        let mut results: Vec<SpeechByCategory> = gender_map
            .into_iter()
            .map(
                |(gender, (total_speeches, total_speech_time, avg_times, delegate_count))| {
                    let average_speech_time = if !avg_times.is_empty() {
                        avg_times.iter().sum::<f64>() / avg_times.len() as f64
                    } else {
                        0.0
                    };

                    SpeechByCategory {
                        category: gender,
                        total_speeches,
                        total_speech_time,
                        average_speech_time,
                        delegate_count,
                    }
                },
            )
            .collect();

        // Sort based on speech type and normalized flag
        match filter.speech_type.as_str() {
            "speechtime" => {
                if filter.normalized {
                    results.sort_by(|a, b| {
                        b.average_speech_time
                            .partial_cmp(&a.average_speech_time)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                } else {
                    results.sort_by(|a, b| b.total_speech_time.cmp(&a.total_speech_time));
                }
            }
            "total_speeches" => {
                results.sort_by(|a, b| b.total_speeches.cmp(&a.total_speeches));
            }
            _ => {
                if filter.normalized {
                    results.sort_by(|a, b| {
                        b.average_speech_time
                            .partial_cmp(&a.average_speech_time)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                } else {
                    results.sort_by(|a, b| b.total_speech_time.cmp(&a.total_speech_time));
                }
            }
        }

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_legis(
        pg: &sqlx::PgPool,
        filter: &SpeechFilter,
    ) -> Result<Vec<SpeechByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        let mut period_map: std::collections::HashMap<String, (i64, i64, Vec<f64>, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let period = item
                .legislative_period
                .unwrap_or_else(|| "Unbekannt".to_string());
            let entry = period_map.entry(period).or_insert((0, 0, Vec::new(), 0));
            entry.0 += item.total_speeches;
            entry.1 += item.total_speech_time;
            if item.average_speech_time > 0.0 {
                entry.2.push(item.average_speech_time);
            }
            entry.3 += 1;
        }

        let mut results: Vec<SpeechByCategory> = period_map
            .into_iter()
            .map(
                |(period, (total_speeches, total_speech_time, avg_times, delegate_count))| {
                    let average_speech_time = if !avg_times.is_empty() {
                        avg_times.iter().sum::<f64>() / avg_times.len() as f64
                    } else {
                        0.0
                    };

                    SpeechByCategory {
                        category: period,
                        total_speeches,
                        total_speech_time,
                        average_speech_time,
                        delegate_count,
                    }
                },
            )
            .collect();

        // Sort based on speech type and normalized flag
        match filter.speech_type.as_str() {
            "speechtime" => {
                if filter.normalized {
                    results.sort_by(|a, b| {
                        b.average_speech_time
                            .partial_cmp(&a.average_speech_time)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                } else {
                    results.sort_by(|a, b| b.total_speech_time.cmp(&a.total_speech_time));
                }
            }
            "total_speeches" => {
                results.sort_by(|a, b| b.total_speeches.cmp(&a.total_speeches));
            }
            _ => {
                if filter.normalized {
                    results.sort_by(|a, b| {
                        b.average_speech_time
                            .partial_cmp(&a.average_speech_time)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                } else {
                    results.sort_by(|a, b| b.total_speech_time.cmp(&a.total_speech_time));
                }
            }
        }

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    pub async fn per_age(
        pg: &sqlx::PgPool,
        filter: &SpeechFilter,
    ) -> Result<Vec<SpeechByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        let mut age_map: std::collections::HashMap<String, (i64, i64, Vec<f64>, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry = age_map
                .entry(item.delegate_age_bucket)
                .or_insert((0, 0, Vec::new(), 0));
            entry.0 += item.total_speeches;
            entry.1 += item.total_speech_time;
            if item.average_speech_time > 0.0 {
                entry.2.push(item.average_speech_time);
            }
            entry.3 += 1;
        }

        let mut results: Vec<SpeechByCategory> = age_map
            .into_iter()
            .map(
                |(category, (total_speeches, total_speech_time, avg_times, delegate_count))| {
                    let average_speech_time = if !avg_times.is_empty() {
                        avg_times.iter().sum::<f64>() / avg_times.len() as f64
                    } else {
                        0.0
                    };

                    SpeechByCategory {
                        category,
                        total_speeches,
                        total_speech_time,
                        average_speech_time,
                        delegate_count,
                    }
                },
            )
            .collect();

        // Sort based on speech type and normalized flag
        match filter.speech_type.as_str() {
            "speechtime" => {
                if filter.normalized {
                    results.sort_by(|a, b| {
                        b.average_speech_time
                            .partial_cmp(&a.average_speech_time)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                } else {
                    results.sort_by(|a, b| b.total_speech_time.cmp(&a.total_speech_time));
                }
            }
            "total_speeches" => {
                results.sort_by(|a, b| b.total_speeches.cmp(&a.total_speeches));
            }
            _ => {
                if filter.normalized {
                    results.sort_by(|a, b| {
                        b.average_speech_time
                            .partial_cmp(&a.average_speech_time)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                } else {
                    results.sort_by(|a, b| b.total_speech_time.cmp(&a.total_speech_time));
                }
            }
        }

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }
}

// Main endpoint functions
pub async fn speechtime_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<SpeechFilter>>,
) -> Result<Json<Vec<SpeechForDelegate>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.speech_type = "speechtime".to_string();
    let results = SpeechService::per_delegate(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn speechtime_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<SpeechFilter>>,
) -> Result<Json<Vec<SpeechByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.speech_type = "speechtime".to_string();
    let results = SpeechService::per_party(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn speechtime_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<SpeechFilter>>,
) -> Result<Json<Vec<SpeechByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.speech_type = "speechtime".to_string();
    let results = SpeechService::per_gender(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn speechtime_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<SpeechFilter>>,
) -> Result<Json<Vec<SpeechByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.speech_type = "speechtime".to_string();
    let results = SpeechService::per_legis(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn speechtime_per_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<SpeechFilter>>,
) -> Result<Json<Vec<SpeechByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.speech_type = "speechtime".to_string();
    let results = SpeechService::per_age(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn total_speeches_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<SpeechFilter>>,
) -> Result<Json<Vec<SpeechForDelegate>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.speech_type = "total_speeches".to_string();
    let results = SpeechService::per_delegate(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn total_speeches_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<SpeechFilter>>,
) -> Result<Json<Vec<SpeechByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.speech_type = "total_speeches".to_string();
    let results = SpeechService::per_party(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn total_speeches_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<SpeechFilter>>,
) -> Result<Json<Vec<SpeechByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.speech_type = "total_speeches".to_string();
    let results = SpeechService::per_gender(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn total_speeches_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<SpeechFilter>>,
) -> Result<Json<Vec<SpeechByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.speech_type = "total_speeches".to_string();
    let results = SpeechService::per_legis(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn total_speeches_per_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<SpeechFilter>>,
) -> Result<Json<Vec<SpeechByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.speech_type = "total_speeches".to_string();
    let results = SpeechService::per_age(&pg, &filter).await?;
    Ok(Json(results))
}
