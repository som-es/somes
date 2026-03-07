use sqlx::{Postgres, Transaction};

pub async fn create_delegates_view<'a>(tx: &mut Transaction<'a, Postgres>) -> sqlx::Result<()> {
    sqlx::query!("DROP VIEW IF EXISTS delegates_with_mandates;")
        .execute(&mut **tx)
        .await?;

    sqlx::query!(
        r#"CREATE OR REPLACE VIEW delegates_with_mandates AS
        WITH period_starts AS (
            SELECT legislative_period AS gp, MIN(raw_data_created_at)::date AS start_date
            FROM plenar_infos
            GROUP BY legislative_period 
            HAVING COUNT(*) > 1
        ),
        periods AS (
            SELECT 
                gp,
                start_date,
                LEAD(start_date) OVER (ORDER BY start_date ASC) AS end_date
            FROM period_starts
        )
        SELECT
            d.id,
            d.name,
            d.party,
            d.party AS current_party,
            d.image_url,
            d.constituency,
            d.council,
            d.seat_row,
            d.seat_col,
            d.gender,
            d.is_active,
            d.birthdate,
            d.created_at,
            d.updated_at,
            
            m_agg.active_mandates AS "mandates_at_time: Vec<FullMandate>",
            
            ARRAY(
                SELECT division 
                FROM delegates_divisions 
                WHERE delegate_id = d.id 
                ORDER BY insertion_date DESC 
                LIMIT 1
            ) AS divisions,
            
            m_agg.all_mandates AS "mandates: Vec<FullMandate>",
            m_agg.active_mandates AS "active_mandates: Vec<FullMandate>",
            
            gp_agg.active_gps AS "active_gps: Vec<String>",
            gp_agg.active_nr_gps AS "active_nr_gps: Vec<String>",
            gp_agg.active_gov_gps AS "active_gov_gps: Vec<String>"

        FROM
            delegates d
            
        LEFT JOIN LATERAL (
            SELECT 
                COALESCE(
                    ARRAY_AGG(ROW(m.start_date, m.end_date, m.name, m.party, m.is_nr, m.is_gov_official, m.is_ministry, m.is_chancellor, m.function)::full_mandate), 
                    '{}'
                ) AS all_mandates,
                
                COALESCE(
                    ARRAY_AGG(ROW(m.start_date, m.end_date, m.name, m.party, m.is_nr, m.is_gov_official, m.is_ministry, m.is_chancellor, m.function)::full_mandate) FILTER (WHERE m.end_date IS NULL), 
                    '{}'
                ) AS active_mandates
                
            FROM mandates m
            WHERE m.delegate_id = d.id
        ) m_agg ON true

        LEFT JOIN LATERAL (
            SELECT 
                COALESCE(ARRAY_AGG(DISTINCT p.gp), '{}') AS active_gps,
                
                COALESCE(ARRAY_AGG(DISTINCT p.gp) FILTER (WHERE m.is_nr = true), '{}') AS active_nr_gps,
                
                COALESCE(ARRAY_AGG(DISTINCT p.gp) FILTER (WHERE m.is_gov_official = true), '{}') AS active_gov_gps
                
            FROM mandates m
            JOIN periods p 
                ON m.start_date <= COALESCE(p.end_date, 'infinity'::date)
            AND COALESCE(m.end_date, 'infinity'::date) >= p.start_date
            WHERE m.delegate_id = d.id
        ) gp_agg ON true;"#
    ).execute(&mut **tx).await?;
    Ok(())
}
