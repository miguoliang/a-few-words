#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),
    #[error("Row not found")]
    RowNotFound,
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("Third-party error: {0}")]
    ThirdParty(String),
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => Error::RowNotFound,
            sqlx::Error::Database(db_err) => {
                if db_err.is_unique_violation() {
                    Error::Conflict(db_err.message().to_string())
                } else {
                    Error::Unexpected(db_err.message().to_string())
                }
            }
            _ => Error::Unexpected(e.to_string()),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ThirdParty(e.to_string())
    }
}
