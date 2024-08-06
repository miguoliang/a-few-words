use once_cell::sync::Lazy;
use regex::Regex;
use sqlx::{prelude::*, types::chrono};
use validator::Validate;

pub const MAX_WORD_LENGTH: u64 = 500;
pub const MAX_PAGE_SIZE: i32 = 100;
pub const MAX_USERNAME_LENGTH: u64 = 100;

static NOT_BLANK: Lazy<Regex> = Lazy::new(|| Regex::new(r"\S+").unwrap());

pub static USERNAME_LIKE: Lazy<Regex> =
    Lazy::new(|| Regex::new(format!("^[a-zA-Z0-9]{{3,{MAX_USERNAME_LENGTH}}}$").as_str()).unwrap());

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Validate)]
pub struct NewWord {
    #[validate(length(min = 1, max = MAX_WORD_LENGTH), regex(path = *NOT_BLANK))]
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
    #[validate(range(min = 1, max = MAX_PAGE_SIZE))]
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

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_not_blank_regex() {
        let valid_cases = vec![
            "username username", // Multiple words
            " username123",      // Leading whitespace
            "username-123 ",     // Trailing whitespace
            "username_123",      // Non-whitespace characters
        ];

        for case in valid_cases {
            assert!(
                NOT_BLANK.is_match(case),
                "Expected validation to pass for input: {}",
                case
            );
        }

        let invalid_cases = vec!["", " ", "  ", "\t", "\n", "\r", "\r\n"];

        for case in invalid_cases {
            assert!(
                !NOT_BLANK.is_match(case),
                "Expected validation to fail for input: {}",
                case
            );
        }
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
