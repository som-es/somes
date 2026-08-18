use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use views::{create_composite_types, create_views};

#[derive(Parser)]
#[command(
    name = "views",
    about = "Drop and (re-)create database views and composite types"
)]
struct Cli {
    /// Drop everything first, then create everything (default)
    #[arg(long)]
    up: bool,
    /// Drop everything
    #[arg(long, conflicts_with = "up")]
    down: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();
    let up = !cli.down;
    let direction = if up { "up" } else { "down" };

    for env_key in ["DATASERVICE_URL", "EU_DATASERVICE_URL"] {
        let url = std::env::var(env_key)?;
        println!("{direction}: {env_key}");

        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(&url)
            .await?;

        let mut tx = pool.begin().await?;
        create_composite_types(&mut tx, up).await?;
        create_views(&mut tx, up).await?;
        tx.commit().await?;
    }

    Ok(())
}
