use error::Error;
use sqlx::PgPool;

pub mod api;
pub mod translate;
pub mod types;

pub mod error;

pub async fn setup_database(pool: &PgPool) -> Result<(), Error> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
