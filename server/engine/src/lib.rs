use sqlx::PgPool;
use types::Error;

pub mod api;
pub mod types;

pub async fn setup_database(pool: &PgPool) -> Result<(), Error> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    Ok(())
}
