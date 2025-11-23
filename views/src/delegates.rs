use sqlx::{PgPool};

pub async fn create_delegates_view(pool: &PgPool) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query!("DROP VIEW IF EXISTS delegates_with_mandates;")
        .execute(&mut *tx)
        .await?;

    sqlx::query!(
        "
    CREATE OR REPLACE VIEW delegates_with_mandates AS
    SELECT
        delegates.id,
        delegates.name,
        delegates.party,
        delegates.party AS current_party,
        delegates.image_url,
        delegates.constituency,
        delegates.council,
        delegates.seat_row,
        delegates.seat_col,
        delegates.gender,
        delegates.is_active,
        delegates.birthdate,
        COALESCE(divisions.division_array, '{}') AS divisions,
        ARRAY(
            SELECT ROW(start_date, end_date, name, party, is_nr, is_gov_official, is_ministry, is_chancellor, function) 
            FROM mandates m 
            where delegate_id = delegates.id and end_date IS NULL
        ) as \"active_mandates: Vec<FullMandate>\"
    FROM
        delegates
    LEFT JOIN
        (SELECT
            delegate_id,
            ARRAY_AGG(division) AS division_array
        FROM
            delegates_divisions
        GROUP BY
            delegate_id) AS divisions
        ON delegates.id = divisions.delegate_id
        "
    ).execute(&mut *tx).await?;

    Ok(())
}