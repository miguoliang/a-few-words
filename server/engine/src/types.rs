use chrono::{Duration, NaiveDateTime};
use once_cell::sync::Lazy;
use regex::Regex;
use sqlx::FromRow;
use validator::Validate;

/// First page of a paginated query
/// This is the default page number to start with
/// The first page is set to 0
pub const FIRST_PAGE: u64 = 0;

/// Default page size of a paginated query
/// This is the number of results to return in a single page
/// The default page size is set to 10
/// If a user wants to retrieve more results, they can specify a larger page size
/// If a user wants to retrieve fewer results, they can specify a smaller page size
pub const DEFAULT_PAGE_SIZE: u64 = 10;

/// Maximum page size of a paginated query
/// This is to prevent a user from requesting too many results at once
/// The maximum page size is set to 100
/// If a user wants to retrieve more results, they can make multiple requests
/// with different page numbers
pub const MAX_PAGE_SIZE: u64 = 100;

/// Maximum length of a word
/// This is the maximum length of a word to be translated
pub const MAX_WORD_LENGTH: u64 = 5000;

/// Maximum length of a definition
/// This is the maximum length of a definition for a word
pub const MAX_DEFINITION_LENGTH: u64 = 5000;

/// Maximum length of a URL
/// This is the maximum length of a URL for a word
pub const MAX_URL_LENGTH: u64 = 5000;

/// Minimum length of a user ID
pub const MIN_USER_ID_LENGTH: usize = 5;

/// Maximum length of a user ID
pub const MAX_USER_ID_LENGTH: usize = 50;

pub static USER_ID_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!(
        r"^[a-zA-Z0-9_]{{{},{}}}$",
        MIN_USER_ID_LENGTH, MAX_USER_ID_LENGTH
    ))
    .unwrap()
});

/// Represents a word entry in the database
#[derive(Debug, FromRow)]
pub struct Word {
    pub word_id: i32,
    pub user_id: String, // Matches the JWT 'username' field
    pub word: String,
    pub definition: String,
    pub url: String,
    pub date_added: NaiveDateTime,
    pub initial_forgetting_rate: f64,
}

/// Represents a new word entry to be inserted into the database
#[derive(Debug, Validate)]
pub struct NewWord {
    #[validate(regex(path = *USER_ID_PATTERN))]
    pub user_id: String,
    #[validate(length(min = 1, max = MAX_WORD_LENGTH))]
    pub word: String,
    #[validate(length(min = 1, max = MAX_DEFINITION_LENGTH))]
    pub definition: String,
    #[validate(url)]
    pub url: String,
    #[validate(range(min = 0.0, max = 1.0))]
    pub initial_forgetting_rate: Option<f64>,
}

impl NewWord {
    pub fn new(user_id: String, word: String, definition: String, url: String) -> Self {
        Self {
            user_id,
            word,
            definition,
            url,
            initial_forgetting_rate: None,
        }
    }

    pub fn with_forgetting_rate(mut self, rate: f64) -> Self {
        self.initial_forgetting_rate = Some(rate);
        self
    }
}

/// Represents a review session for a word
#[derive(Debug, FromRow)]
pub struct ReviewSession {
    pub session_id: i32,
    pub word_id: i32, // Foreign key from the words table
    pub review_date: NaiveDateTime,
    pub recall_score: i32, // Scale from 1 to 5
    pub time_to_forget: Option<Duration>,
    pub next_review_date: Option<NaiveDateTime>,
}

/// Represents a new review session entry to be inserted into the database
#[derive(Debug, Validate)]
pub struct NewReviewSession {
    #[validate(range(min = 1))]
    pub word_id: i32,
    #[validate(range(min = 1, max = 5))]
    pub recall_score: i32,
    pub time_to_forget: Option<Duration>,
    pub next_review_date: Option<NaiveDateTime>,
}

impl NewReviewSession {
    pub fn new(word_id: i32, recall_score: i32) -> Self {
        Self {
            word_id,
            recall_score,
            time_to_forget: None,
            next_review_date: None,
        }
    }

    pub fn with_time_to_forget(mut self, time_to_forget: Duration) -> Self {
        self.time_to_forget = Some(time_to_forget);
        self
    }

    pub fn with_next_review_date(mut self, next_review_date: NaiveDateTime) -> Self {
        self.next_review_date = Some(next_review_date);
        self
    }
}

/// Represents the forgetting curve for a word
#[derive(Debug, FromRow)]
pub struct ForgettingCurve {
    pub curve_id: i32,
    pub word_id: i32, // Foreign key from the words table
    pub review_interval: Option<Duration>,
    pub retention_rate: f64, // Between 0 and 1
    pub review_count: i32,   // Number of reviews done
}

/// Represents a new forgetting curve entry to be inserted into the database
#[derive(Debug, Validate)]
pub struct NewForgettingCurve {
    #[validate(range(min = 1))]
    pub word_id: i32,
    #[validate(range(min = 0.0, max = 1.0))]
    pub retention_rate: f64,
    #[validate(range(min = 1))]
    pub review_count: i32,
}
