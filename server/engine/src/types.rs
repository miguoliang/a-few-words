use once_cell::sync::Lazy;
use regex::Regex;
use sqlx::{prelude::*, types::chrono};
use validator::Validate;

static NOT_BLANK: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\S+$").unwrap());

pub static USERNAME_LIKE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9]{3,100}$").unwrap());

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Validate)]
pub struct NewWord {
    #[validate(length(min = 1, max = 100), regex(path = *NOT_BLANK))]
    pub word: String,
    #[validate(url)]
    pub url: Option<String>,
    #[validate(regex(path = *USERNAME_LIKE))]
    pub username: String,
}

#[cfg_attr(feature = "serde", derive(Deserialize))]
#[derive(Validate)]
pub struct Offset {
    #[validate(range(min = 0))]
    pub offset: Option<i32>,
    #[validate(range(min = 1, max = 100))]
    pub size: Option<i32>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(FromRow, PartialEq, Debug)]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub url: Option<String>,
    pub username: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

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

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_not_blank_regex() {
        assert_ne!(NOT_BLANK.is_match("username username"), true);
        assert_ne!(NOT_BLANK.is_match(" "), true);
        assert_ne!(NOT_BLANK.is_match(""), true);
    }

    #[tokio::test]
    async fn test_valid_username() {
        let invalid_cases = vec!["username", "username123"];

        for case in invalid_cases {
            assert!(
                USERNAME_LIKE.is_match(case),
                "Expected validation to fail for input: {}",
                case
            );
        }
    }

    #[tokio::test]
    async fn test_invalid_fields() {
        // Test cases that should fail the validation
        let binding = ["a"; 101].join("");
        let too_long = binding.as_str();
        let invalid_cases = vec![
            "",          // Empty string
            "ab",        // Too short
            "a@b",       // Invalid character @ in invalid position
            "a-b",       // Invalid character - in invalid position
            "a.b",       // Invalid character . in invalid position
            "abc!",      // Invalid character !
            "abc$",      // Invalid character $
            " abc",      // Leading whitespace
            "abc ",      // Trailing whitespace
            "abc def",   // Space inside
            "abc@def@g", // Multiple @ symbols
            too_long,    // Too long
        ];

        for case in invalid_cases {
            assert!(
                !USERNAME_LIKE.is_match(case),
                "Expected validation to fail for input: {}",
                case
            );
        }
    }
}
