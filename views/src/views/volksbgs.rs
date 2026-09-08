use sqlx::{Postgres, Transaction};

pub async fn create_volksbgs_view<'a>(
    tx: &mut Transaction<'a, Postgres>,
    up: bool,
) -> sqlx::Result<()> {
    sqlx::query!("DROP VIEW IF EXISTS volksbg_weeks;")
        .execute(&mut **tx)
        .await?;
    if up {
        sqlx::query!(
            r#"
                CREATE VIEW volksbg_weeks AS
                SELECT
                    w.id,
                    w.start_date,
                    w.end_date,
                    w.cut_off_date,
                    w.online_deadline_utc,
                    w.polling_stations_url,
                    ARRAY(
                        SELECT ROW(
                            v.id,
                            v.slug,
                            v.title,
                            v.description,
                            v.overview_url,
                            v.state,
                            v.ruling_date,
                            v.cut_off_date,
                            v.eintragungswoche,
                            ARRAY(
                                SELECT ROW(
                                    doc.title,
                                    doc.document_url,
                                    doc.document_type
                                )::document
                                FROM volksbg_documents doc
                                WHERE doc.volksbg_id = v.id
                            )
                        )::db_volksbg
                        FROM volksbg v
                    ) AS "volksbgs: Vec<DbVolksbg>"
                FROM volksbg_eintragungswoche w
            "#
        )
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}
