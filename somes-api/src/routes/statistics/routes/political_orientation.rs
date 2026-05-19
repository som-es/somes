use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Postgres};
use utoipa::ToSchema;

use crate::{
    routes::statistics::routes::error::StatisticsResponse,
    routes::statistics::routes::filtering::{bind_values, build_filter, IntoFilterArgument},
    PgPoolConnection,
};

#[derive(ToSchema, Default, Debug, Clone, Serialize, Deserialize)]
pub struct PoliticalOrientationFilter {
    legis_period: Option<String>,
    gender: Option<String>,
    party: Option<String>,
    is_desc: bool,
    #[serde(default)]
    orientation_type: String, // "left", "right", "liberal" or "authoritarian"
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PoliticalOrientationBase {
    delegate_name: String,
    delegate_party: String,
    delegate_gender: String,
    orientation_score: f64,
    total_votes: i64,
    delegate_age_bucket: String,
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

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PoliticalSpectrumBase {
    delegate_name: String,
    delegate_party: String,
    delegate_gender: String,
    left_right_score: f64,
    liberal_authoritarian_score: f64,
    total_votes: i64,
    delegate_age_bucket: String,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PoliticalSpectrumForDelegate {
    delegate_name: String,
    delegate_party: String,
    left_right_score: f64,
    liberal_authoritarian_score: f64,
    spectrum_magnitude: f64,
    total_votes: i64,
}

#[derive(ToSchema, PartialEq, Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PoliticalSpectrumByCategory {
    category: String,
    average_left_right_score: f64,
    average_liberal_authoritarian_score: f64,
    spectrum_magnitude: f64,
    total_votes: i64,
    delegate_count: i64,
}

pub struct PoliticalOrientationService;

impl PoliticalOrientationService {
    fn sort_categories(results: &mut [PoliticalOrientationByCategory], is_desc: bool) {
        results.sort_by(|a, b| {
            b.average_orientation
                .partial_cmp(&a.average_orientation)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !is_desc {
            results.reverse();
        }
    }

    fn aggregate_by_party(
        base_data: Vec<PoliticalOrientationBase>,
        is_desc: bool,
    ) -> Vec<PoliticalOrientationByCategory> {
        let mut party_map: std::collections::HashMap<String, (Vec<f64>, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry = party_map
                .entry(item.delegate_party.clone())
                .or_insert((Vec::new(), 0, 0));
            entry.0.push(item.orientation_score);
            entry.1 += item.total_votes;
            entry.2 += 1;
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

        Self::sort_categories(&mut results, is_desc);
        results
    }

    fn aggregate_by_gender(
        base_data: Vec<PoliticalOrientationBase>,
        is_desc: bool,
    ) -> Vec<PoliticalOrientationByCategory> {
        let mut gender_map: std::collections::HashMap<String, (Vec<f64>, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry =
                gender_map
                    .entry(item.delegate_gender.clone())
                    .or_insert((Vec::new(), 0, 0));
            entry.0.push(item.orientation_score);
            entry.1 += item.total_votes;
            entry.2 += 1;
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

        Self::sort_categories(&mut results, is_desc);
        results
    }

    fn aggregate_by_age(
        base_data: Vec<PoliticalOrientationBase>,
        is_desc: bool,
    ) -> Vec<PoliticalOrientationByCategory> {
        let mut age_map: std::collections::HashMap<String, (Vec<f64>, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry = age_map
                .entry(item.delegate_age_bucket)
                .or_insert((Vec::new(), 0, 0));
            entry.0.push(item.orientation_score);
            entry.1 += item.total_votes;
            entry.2 += 1;
        }

        let mut results: Vec<PoliticalOrientationByCategory> = age_map
            .into_iter()
            .map(|(age, (scores, total_votes, delegate_count))| {
                let average_orientation = if scores.is_empty() {
                    0.0
                } else {
                    scores.iter().sum::<f64>() / scores.len() as f64
                };

                PoliticalOrientationByCategory {
                    category: age,
                    average_orientation,
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
        filter: &PoliticalOrientationFilter,
    ) -> Result<Vec<PoliticalOrientationBase>, StatisticsResponse> {
        let orientation_column = match filter.orientation_type.as_str() {
            "right" => "pp.is_not_left",
            "liberal" => "pp.is_liberal",
            "authoritarian" => "pp.is_not_liberal",
            _ => "pp.is_left",
        };

        let query = format!(
            "
        WITH period_bounds AS (
            SELECT
                pf.legislative_period AS gp,
                MIN(pf.raw_data_created_at)::date AS start_date,
                MAX(pf.raw_data_created_at)::date AS end_date,
                MAX(pf.raw_data_created_at)::date AS reference_date
            FROM plenar_infos pf
            GROUP BY pf.legislative_period
        )
        SELECT
            d.name AS delegate_name,
            COALESCE(active_mandate.party, d.party, 'Regierungsmitglied') AS delegate_party,
            COALESCE(d.gender, '') AS delegate_gender,
            {}::float8 AS orientation_score,
            pp.neutral_count::bigint AS total_votes,
            CASE
                WHEN d.birthdate IS NULL THEN 'Unbekannt'
                WHEN EXTRACT(YEAR FROM AGE(COALESCE(pb.reference_date, CURRENT_DATE), d.birthdate)) <= 30 THEN '18-30'
                WHEN EXTRACT(YEAR FROM AGE(COALESCE(pb.reference_date, CURRENT_DATE), d.birthdate)) <= 40 THEN '31-40'
                WHEN EXTRACT(YEAR FROM AGE(COALESCE(pb.reference_date, CURRENT_DATE), d.birthdate)) <= 50 THEN '41-50'
                WHEN EXTRACT(YEAR FROM AGE(COALESCE(pb.reference_date, CURRENT_DATE), d.birthdate)) <= 60 THEN '51-60'
                ELSE '60+'
            END AS delegate_age_bucket
        FROM political_positions pp
        JOIN delegates d ON pp.delegate_id = d.id
        LEFT JOIN period_bounds pb ON pb.gp = $1
        LEFT JOIN LATERAL (
            SELECT m.id, m.party
            FROM mandates m
            WHERE m.delegate_id = d.id
                AND (m.is_nr OR m.is_gov_official)
                AND (
                    $1::text IS NULL
                    OR (
                        m.start_date <= pb.end_date
                        AND (m.end_date IS NULL OR m.end_date >= pb.start_date)
                    )
                )
            ORDER BY
                CASE
                    WHEN $1::text IS NULL THEN COALESCE(m.end_date, 'infinity'::date)
                    ELSE LEAST(COALESCE(m.end_date, pb.end_date), pb.end_date)
                END DESC,
                m.start_date DESC
            LIMIT 1
        ) active_mandate ON true
        WHERE
            ($1::text IS NULL OR active_mandate.id IS NOT NULL)
            AND ($2::text IS NULL OR d.gender = $2)
            AND (
                $3::text IS NULL
                OR COALESCE(active_mandate.party, d.party, 'Regierungsmitglied') = $3
            )
        ORDER BY
            orientation_score DESC;
        ",
            orientation_column
        );

        let filtered_query = sqlx::query_as::<Postgres, PoliticalOrientationBase>(&query)
            .bind(filter.legis_period.as_deref())
            .bind(filter.gender.as_deref())
            .bind(filter.party.as_deref());

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
        Ok(Self::aggregate_by_party(base_data, filter.is_desc))
    }

    pub async fn per_gender(
        pg: &sqlx::PgPool,
        filter: &PoliticalOrientationFilter,
    ) -> Result<Vec<PoliticalOrientationByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        Ok(Self::aggregate_by_gender(base_data, filter.is_desc))
    }

    pub async fn per_legis(
        pg: &sqlx::PgPool,
        filter: &PoliticalOrientationFilter,
    ) -> Result<Vec<PoliticalOrientationByCategory>, StatisticsResponse> {
        let orientation_column = match filter.orientation_type.as_str() {
            "right" => "pp.is_not_left",
            "liberal" => "pp.is_liberal",
            "authoritarian" => "pp.is_not_liberal",
            _ => "pp.is_left",
        };

        let query = format!(
            "
        WITH period_bounds AS (
            SELECT
                pf.legislative_period AS gp,
                MIN(pf.raw_data_created_at)::date AS start_date,
                MAX(pf.raw_data_created_at)::date AS end_date
            FROM plenar_infos pf
            GROUP BY pf.legislative_period
        )
        SELECT
            pb.gp AS category,
            AVG({}::float8) AS average_orientation,
            SUM(pp.neutral_count)::bigint AS total_votes,
            COUNT(DISTINCT d.id) AS delegate_count
        FROM period_bounds pb
        JOIN political_positions pp ON true
        JOIN delegates d ON pp.delegate_id = d.id
        JOIN LATERAL (
            SELECT m.id, m.party
            FROM mandates m
            WHERE m.delegate_id = d.id
                AND (m.is_nr OR m.is_gov_official)
                AND m.start_date <= pb.end_date
                AND (m.end_date IS NULL OR m.end_date >= pb.start_date)
            ORDER BY
                LEAST(COALESCE(m.end_date, pb.end_date), pb.end_date) DESC,
                m.start_date DESC
            LIMIT 1
        ) active_mandate ON true
        WHERE
            ($1::text IS NULL OR pb.gp = $1)
            AND ($2::text IS NULL OR d.gender = $2)
            AND ($3::text IS NULL OR COALESCE(active_mandate.party, d.party, 'Regierungsmitglied') = $3)
        GROUP BY
            pb.gp
        ORDER BY
            average_orientation DESC;
        ",
            orientation_column
        );

        let filtered_query = sqlx::query_as::<Postgres, PoliticalOrientationByCategory>(&query)
            .bind(filter.legis_period.as_deref())
            .bind(filter.gender.as_deref())
            .bind(filter.party.as_deref());

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
        Ok(Self::aggregate_by_age(base_data, filter.is_desc))
    }
}

pub struct PoliticalSpectrumService;

impl PoliticalSpectrumService {
    pub async fn get_base_data(
        pg: &sqlx::PgPool,
        filter: &PoliticalOrientationFilter,
    ) -> Result<Vec<PoliticalSpectrumBase>, StatisticsResponse> {
        let query = "
        WITH period_bounds AS (
            SELECT
                pf.legislative_period AS gp,
                MIN(pf.raw_data_created_at)::date AS start_date,
                MAX(pf.raw_data_created_at)::date AS end_date,
                MAX(pf.raw_data_created_at)::date AS reference_date
            FROM plenar_infos pf
            GROUP BY pf.legislative_period
        )
        SELECT
            d.name AS delegate_name,
            COALESCE(active_mandate.party, d.party, 'Regierungsmitglied') AS delegate_party,
            COALESCE(d.gender, '') AS delegate_gender,
            (pp.is_not_left::float8 - pp.is_left::float8) AS left_right_score,
            (pp.is_not_liberal::float8 - pp.is_liberal::float8) AS liberal_authoritarian_score,
            pp.neutral_count::bigint AS total_votes,
            CASE
                WHEN d.birthdate IS NULL THEN 'Unbekannt'
                WHEN EXTRACT(YEAR FROM AGE(COALESCE(pb.reference_date, CURRENT_DATE), d.birthdate)) <= 30 THEN '18-30'
                WHEN EXTRACT(YEAR FROM AGE(COALESCE(pb.reference_date, CURRENT_DATE), d.birthdate)) <= 40 THEN '31-40'
                WHEN EXTRACT(YEAR FROM AGE(COALESCE(pb.reference_date, CURRENT_DATE), d.birthdate)) <= 50 THEN '41-50'
                WHEN EXTRACT(YEAR FROM AGE(COALESCE(pb.reference_date, CURRENT_DATE), d.birthdate)) <= 60 THEN '51-60'
                ELSE '60+'
            END AS delegate_age_bucket
        FROM political_positions pp
        JOIN delegates d ON pp.delegate_id = d.id
        LEFT JOIN period_bounds pb ON pb.gp = $1
        LEFT JOIN LATERAL (
            SELECT m.id, m.party
            FROM mandates m
            WHERE m.delegate_id = d.id
                AND (m.is_nr OR m.is_gov_official)
                AND (
                    $1::text IS NULL
                    OR (
                        m.start_date <= pb.end_date
                        AND (m.end_date IS NULL OR m.end_date >= pb.start_date)
                    )
                )
            ORDER BY
                CASE
                    WHEN $1::text IS NULL THEN COALESCE(m.end_date, 'infinity'::date)
                    ELSE LEAST(COALESCE(m.end_date, pb.end_date), pb.end_date)
                END DESC,
                m.start_date DESC
            LIMIT 1
        ) active_mandate ON true
        WHERE
            ($1::text IS NULL OR active_mandate.id IS NOT NULL)
            AND ($2::text IS NULL OR d.gender = $2)
            AND (
                $3::text IS NULL
                OR COALESCE(active_mandate.party, d.party, 'Regierungsmitglied') = $3
            );
        ";

        sqlx::query_as::<Postgres, PoliticalSpectrumBase>(query)
            .bind(filter.legis_period.as_deref())
            .bind(filter.gender.as_deref())
            .bind(filter.party.as_deref())
            .fetch_all(pg)
            .await
            .map_err(|e| StatisticsResponse::DbSelectFailure(Some(e)))
    }

    fn magnitude(left_right_score: f64, liberal_authoritarian_score: f64) -> f64 {
        (left_right_score.powi(2) + liberal_authoritarian_score.powi(2)).sqrt()
    }

    pub async fn per_delegate(
        pg: &sqlx::PgPool,
        filter: &PoliticalOrientationFilter,
    ) -> Result<Vec<PoliticalSpectrumForDelegate>, StatisticsResponse> {
        let mut results: Vec<PoliticalSpectrumForDelegate> = Self::get_base_data(pg, filter)
            .await?
            .into_iter()
            .map(|item| {
                let spectrum_magnitude =
                    Self::magnitude(item.left_right_score, item.liberal_authoritarian_score);
                PoliticalSpectrumForDelegate {
                    delegate_name: item.delegate_name,
                    delegate_party: item.delegate_party,
                    left_right_score: item.left_right_score,
                    liberal_authoritarian_score: item.liberal_authoritarian_score,
                    spectrum_magnitude,
                    total_votes: item.total_votes,
                }
            })
            .collect();

        results.sort_by(|a, b| {
            b.spectrum_magnitude
                .partial_cmp(&a.spectrum_magnitude)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !filter.is_desc {
            results.reverse();
        }

        Ok(results)
    }

    fn aggregate_category(
        grouped: std::collections::HashMap<String, (Vec<f64>, Vec<f64>, i64, i64)>,
        is_desc: bool,
    ) -> Vec<PoliticalSpectrumByCategory> {
        let mut results: Vec<PoliticalSpectrumByCategory> = grouped
            .into_iter()
            .map(
                |(
                    category,
                    (left_right_scores, liberal_authoritarian_scores, total_votes, delegate_count),
                )| {
                    let average_left_right_score = if left_right_scores.is_empty() {
                        0.0
                    } else {
                        left_right_scores.iter().sum::<f64>() / left_right_scores.len() as f64
                    };
                    let average_liberal_authoritarian_score =
                        if liberal_authoritarian_scores.is_empty() {
                            0.0
                        } else {
                            liberal_authoritarian_scores.iter().sum::<f64>()
                                / liberal_authoritarian_scores.len() as f64
                        };
                    let spectrum_magnitude = Self::magnitude(
                        average_left_right_score,
                        average_liberal_authoritarian_score,
                    );

                    PoliticalSpectrumByCategory {
                        category,
                        average_left_right_score,
                        average_liberal_authoritarian_score,
                        spectrum_magnitude,
                        total_votes,
                        delegate_count,
                    }
                },
            )
            .collect();

        results.sort_by(|a, b| {
            b.spectrum_magnitude
                .partial_cmp(&a.spectrum_magnitude)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if !is_desc {
            results.reverse();
        }

        results
    }

    fn aggregate_by_party(
        base_data: Vec<PoliticalSpectrumBase>,
        is_desc: bool,
    ) -> Vec<PoliticalSpectrumByCategory> {
        let mut grouped: std::collections::HashMap<String, (Vec<f64>, Vec<f64>, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry =
                grouped
                    .entry(item.delegate_party)
                    .or_insert((Vec::new(), Vec::new(), 0, 0));
            entry.0.push(item.left_right_score);
            entry.1.push(item.liberal_authoritarian_score);
            entry.2 += item.total_votes;
            entry.3 += 1;
        }

        Self::aggregate_category(grouped, is_desc)
    }

    fn aggregate_by_gender(
        base_data: Vec<PoliticalSpectrumBase>,
        is_desc: bool,
    ) -> Vec<PoliticalSpectrumByCategory> {
        let mut grouped: std::collections::HashMap<String, (Vec<f64>, Vec<f64>, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry =
                grouped
                    .entry(item.delegate_gender)
                    .or_insert((Vec::new(), Vec::new(), 0, 0));
            entry.0.push(item.left_right_score);
            entry.1.push(item.liberal_authoritarian_score);
            entry.2 += item.total_votes;
            entry.3 += 1;
        }

        Self::aggregate_category(grouped, is_desc)
    }

    fn aggregate_by_age(
        base_data: Vec<PoliticalSpectrumBase>,
        is_desc: bool,
    ) -> Vec<PoliticalSpectrumByCategory> {
        let mut grouped: std::collections::HashMap<String, (Vec<f64>, Vec<f64>, i64, i64)> =
            std::collections::HashMap::new();

        for item in base_data {
            let entry =
                grouped
                    .entry(item.delegate_age_bucket)
                    .or_insert((Vec::new(), Vec::new(), 0, 0));
            entry.0.push(item.left_right_score);
            entry.1.push(item.liberal_authoritarian_score);
            entry.2 += item.total_votes;
            entry.3 += 1;
        }

        Self::aggregate_category(grouped, is_desc)
    }

    pub async fn per_party(
        pg: &sqlx::PgPool,
        filter: &PoliticalOrientationFilter,
    ) -> Result<Vec<PoliticalSpectrumByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        Ok(Self::aggregate_by_party(base_data, filter.is_desc))
    }

    pub async fn per_gender(
        pg: &sqlx::PgPool,
        filter: &PoliticalOrientationFilter,
    ) -> Result<Vec<PoliticalSpectrumByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        Ok(Self::aggregate_by_gender(base_data, filter.is_desc))
    }

    pub async fn per_age(
        pg: &sqlx::PgPool,
        filter: &PoliticalOrientationFilter,
    ) -> Result<Vec<PoliticalSpectrumByCategory>, StatisticsResponse> {
        let base_data = Self::get_base_data(pg, filter).await?;
        Ok(Self::aggregate_by_age(base_data, filter.is_desc))
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
    Ok(results)
}

pub async fn is_left_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationForDelegate>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "left".to_string();
    let results = PoliticalOrientationService::per_delegate(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn is_left_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "left".to_string();
    let results = PoliticalOrientationService::per_party(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn is_left_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "left".to_string();
    let results = PoliticalOrientationService::per_gender(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn is_left_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "left".to_string();
    let results = PoliticalOrientationService::per_legis(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn is_left_per_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "left".to_string();
    let results = PoliticalOrientationService::per_age(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn is_right_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationForDelegate>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "right".to_string();
    let results = PoliticalOrientationService::per_delegate(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn is_right_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "right".to_string();
    let results = PoliticalOrientationService::per_party(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn is_right_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "right".to_string();
    let results = PoliticalOrientationService::per_gender(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn is_right_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "right".to_string();
    let results = PoliticalOrientationService::per_legis(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn is_right_per_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "right".to_string();
    let results = PoliticalOrientationService::per_age(&pg, &filter).await?;
    Ok(Json(results))
}

// Legacy endpoint functions for backward compatibility - Is Liberal
pub async fn is_liberal_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationForDelegate>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "liberal".to_string();
    let results = PoliticalOrientationService::per_delegate(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn is_liberal_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "liberal".to_string();
    let results = PoliticalOrientationService::per_party(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn is_liberal_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "liberal".to_string();
    let results = PoliticalOrientationService::per_gender(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn is_liberal_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "liberal".to_string();
    let results = PoliticalOrientationService::per_legis(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn is_liberal_per_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "liberal".to_string();
    let results = PoliticalOrientationService::per_age(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn is_authoritarian_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationForDelegate>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "authoritarian".to_string();
    let results = PoliticalOrientationService::per_delegate(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn is_authoritarian_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "authoritarian".to_string();
    let results = PoliticalOrientationService::per_party(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn is_authoritarian_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "authoritarian".to_string();
    let results = PoliticalOrientationService::per_gender(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn is_authoritarian_per_legis(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "authoritarian".to_string();
    let results = PoliticalOrientationService::per_legis(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn is_authoritarian_per_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalOrientationByCategory>>, StatisticsResponse> {
    let mut filter = filter.unwrap_or_default();
    filter.orientation_type = "authoritarian".to_string();
    let results = PoliticalOrientationService::per_age(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn political_spectrum_per_delegate(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalSpectrumForDelegate>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = PoliticalSpectrumService::per_delegate(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn political_spectrum_per_party(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalSpectrumByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = PoliticalSpectrumService::per_party(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn political_spectrum_per_gender(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalSpectrumByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = PoliticalSpectrumService::per_gender(&pg, &filter).await?;
    Ok(Json(results))
}

pub async fn political_spectrum_per_age(
    PgPoolConnection(pg): PgPoolConnection,
    Json(filter): Json<Option<PoliticalOrientationFilter>>,
) -> Result<Json<Vec<PoliticalSpectrumByCategory>>, StatisticsResponse> {
    let filter = filter.unwrap_or_default();
    let results = PoliticalSpectrumService::per_age(&pg, &filter).await?;
    Ok(Json(results))
}

#[cfg(test)]
#[path = "tests/political_orientation.rs"]
mod tests;
