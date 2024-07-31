use once_cell::sync::Lazy;
use regex::Regex;
use sqlx::{prelude::*, types::chrono};
use validator::Validate;

static NOT_BLANK: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\S+$").unwrap());

static USERNAME_LIKE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9]{3,100}$").unwrap());

#[derive(Validate)]
pub struct NewWord {
    #[validate(length(min = 1, max = 100), regex(path = *NOT_BLANK))]
    pub word: String,
    #[validate(url)]
    pub url: Option<String>,
    #[validate(regex(path = *USERNAME_LIKE))]
    pub username: String,
}

#[derive(Validate)]
pub struct Offset {
    #[validate(range(min = 0))]
    pub offset: Option<i32>,
    #[validate(range(min = 1, max = 100))]
    pub size: Option<i32>,
}

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
