use sqlx::PgPool;
use anyhow::Result;

pub mod api;
pub mod types;

pub async fn setup_database(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
