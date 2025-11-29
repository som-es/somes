use sqlx::{Postgres, Transaction};

pub async fn create_delegates_view<'a>(tx: &mut Transaction<'a, Postgres>) -> sqlx::Result<()> {
    sqlx::query!("DROP VIEW IF EXISTS delegates_with_mandates;")
        .execute(&mut **tx)
        .await?;

    sqlx::query!(
        "
    CREATE VIEW delegates_with_mandates AS
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
        ARRAY(
                    SELECT ROW(start_date, end_date, name, party, is_nr, is_gov_official, is_ministry, is_chancellor, function)::full_mandate
                    FROM mandates m
                    where delegate_id = delegates.id and end_date IS NULL
                ) as \"mandates_at_time: Vec<FullMandate>\",
        ARRAY(
            select division from delegates_divisions where delegate_id = delegates.id
        ) as divisions,
        ARRAY(
            SELECT ROW(start_date, end_date, name, party, is_nr, is_gov_official, is_ministry, is_chancellor, function)::full_mandate
            FROM mandates m
            where delegate_id = delegates.id
        ) as \"mandates: Vec<FullMandate>\",
        ARRAY(
            SELECT ROW(start_date, end_date, name, party, is_nr, is_gov_official, is_ministry, is_chancellor, function)::full_mandate
            FROM mandates m
            where delegate_id = delegates.id and end_date IS NULL
        ) as \"active_mandates: Vec<FullMandate>\"
    FROM
        delegates
        "
    ).execute(&mut **tx).await?;
    Ok(())
}
