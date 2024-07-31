use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::{types::chrono, FromRow};
use validator::Validate;

static WORD_LIKE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\S+").unwrap());

#[derive(Serialize, Deserialize, Validate)]
pub struct WordNew {
    #[validate(length(min = 1, max = 100), regex(path = *WORD_LIKE))]
    pub word: String,
    #[validate(url)]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, Validate)]
pub struct Offset {
    #[validate(range(min = 0))]
    pub offset: Option<i32>,
    #[validate(range(min = 1, max = 100))]
    pub size: Option<i32>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_create_word_validates() {
        // Valid word with URL starting with https
        let word = WordNew {
            word: "hello".to_string(),
            url: Some("https://example.com".to_string()),
        };
        assert!(word.validate().is_ok());

        // Valid word with URL starting with http
        let word = WordNew {
            word: "hello".to_string(),
            url: Some("http://example.com".to_string()),
        };
        assert!(word.validate().is_ok());

        let word = WordNew {
            word: "hello".to_string(),
            url: Some("file:///example.pdf".to_string()),
        };
        assert!(word.validate().is_ok());

        let word = WordNew {
            word: "hello".to_string(),
            url: None,
        };
        assert!(word.validate().is_ok());

        let word = WordNew {
            word: "".to_string(),
            url: None,
        };
        assert!(word.validate().is_err());

        let word = WordNew {
            word: " ".to_string(),
            url: None,
        };
        assert!(word.validate().is_err());

        let word = WordNew {
            word: "hello".to_string(),
            url: Some("example.com".to_string()),
        };
        assert!(word.validate().is_err());
    }
}
