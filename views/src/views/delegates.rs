use sqlx::{Postgres, Transaction};

pub async fn create_delegates_view<'a>(tx: &mut Transaction<'a, Postgres>) -> sqlx::Result<()> {
    sqlx::query!("DROP MATERIALIZED VIEW IF EXISTS delegates_with_mandates;")
        .execute(&mut **tx)
        .await?;

    sqlx::query!(
        r#"CREATE MATERIALIZED VIEW delegates_with_mandates AS
    WITH period_starts AS (
        SELECT legislative_period AS gp,
        (MIN(raw_data_created_at) AT TIME ZONE 'Europe/Vienna')::date AS start_date
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
        delegates.id,
        delegates.name,
        delegates.party,
        delegates.party AS current_party,
        delegates.image_url,
        delegates.image_copyright,
        delegates.constituency,
        delegates.council,
        delegates.seat_row,
        delegates.seat_col,
        delegates.gender,
        delegates.is_active,
        delegates.birthdate,
        delegates.created_at,
        delegates.updated_at,
        ARRAY(
            SELECT ROW(start_date, end_date, name, party, is_nr, is_gov_official, is_ministry, is_chancellor, function)::full_mandate
            FROM mandates m
            where delegate_id = delegates.id and end_date IS NULL
        ) as "mandates_at_time: Vec<FullMandate>",
        ARRAY(
            select division from delegates_divisions where delegate_id = delegates.id order by insertion_date desc limit 1
        ) as divisions,
        ARRAY(
            SELECT ROW(start_date, end_date, name, party, is_nr, is_gov_official, is_ministry, is_chancellor, function)::full_mandate
            FROM mandates m
            where delegate_id = delegates.id
        ) as "mandates: Vec<FullMandate>",
        ARRAY(
            SELECT ROW(start_date, end_date, name, party, is_nr, is_gov_official, is_ministry, is_chancellor, function)::full_mandate
            FROM mandates m
            where delegate_id = delegates.id and end_date IS NULL
        ) as "active_mandates: Vec<FullMandate>",

        ARRAY(
            SELECT DISTINCT p.gp
            FROM mandates m
            JOIN periods p
                ON m.start_date <= COALESCE(p.end_date, 'infinity'::date)
               AND COALESCE(m.end_date, 'infinity'::date) >= p.start_date
            WHERE m.delegate_id = delegates.id
        ) as "active_gps: Vec<String>",
        ARRAY(
            SELECT DISTINCT p.gp
            FROM mandates m
            JOIN periods p
                ON m.start_date <= COALESCE(p.end_date, 'infinity'::date)
               AND COALESCE(m.end_date, 'infinity'::date) >= p.start_date
            WHERE m.delegate_id = delegates.id AND m.is_nr = true
        ) as "active_nr_gps: Vec<String>",
        ARRAY(
            SELECT DISTINCT p.gp
            FROM mandates m
            JOIN periods p
                ON m.start_date <= COALESCE(p.end_date, 'infinity'::date)
               AND COALESCE(m.end_date, 'infinity'::date) >= p.start_date
            WHERE m.delegate_id = delegates.id AND m.is_gov_official = true
        ) as "active_gov_gps: Vec<String>"

    FROM
        delegates;
        "#
    ).execute(&mut **tx).await?;

    sqlx::query!(
        "
        CREATE UNIQUE INDEX idx_delegates_with_mandates_id ON delegates_with_mandates(id);
    "
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}
