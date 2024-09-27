use chrono::{Duration, NaiveDateTime};
use once_cell::sync::Lazy;
use regex::Regex;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::FromRow;
use validator::Validate;

/// Create a regular expression pattern for validating a User ID
/// The User ID must be a valid email address or a username
/// The User ID must be at least 5 characters long
/// The User ID must be at most 255 characters long
/// The User ID must not contain any special characters
/// The User ID must not contain any whitespace characters
/// The User ID must not contain any control characters
/// The User ID must not contain any non-ASCII characters
/// The User ID must not contain any  non-printable characters
/// The User ID must not contain any non-Unicode characters
pub static USER_ID_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?:[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})|([a-zA-Z0-9]{5,255})$")
        .unwrap()
});

/// Represents a word, sentence, or paragraph to be translated
/// The text to be translated must be at least 1 character long
/// The text to be translated must be at most 1000 characters long
/// The text to be translated must not contain any control characters
/// The text to be translated must not contain any non-ASCII characters
/// The text to be translated must not contain any non-printable characters
/// The text to be translated must not contain any non-Unicode characters
/// The text to be translated must not lead or trail with whitespace characters
/// The text to be translated must not contain any consecutive whitespace characters
pub static TRANSLATE_TEXT_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[^\p{Cc}\p{Cn}\p{Cf}\p{Z}]{1,1000}$").unwrap());

/// Represents a piece of text
/// The text must be at least 1 character long
/// The text must be at most 3000 characters long
/// The text must not contain any control characters
/// The text must lead or trail with whitespace characters
/// The text must not contain any consecutive whitespace characters
static TEXT_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*[^\p{Cc}]{1,3000}\s*$").unwrap());

/// Represents a word entry in the database
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Validate)]
pub struct NewWord {
    #[validate(regex(path = *USER_ID_PATTERN))]
    pub user_id: String,
    #[validate(regex(path = *TRANSLATE_TEXT_PATTERN))]
    pub word: String,
    #[validate(regex(path = *TEXT_PATTERN))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, FromRow)]
pub struct ReviewSession {
    pub session_id: i32,
    pub word_id: i32, // Foreign key from the words table
    pub review_date: NaiveDateTime,
    pub recall_score: i32, // Scale from 1 to 5
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_duration",
            deserialize_with = "deserialize_duration"
        )
    )] // Serialize and deserialize as number of seconds
    pub time_to_forget: Option<Duration>,
    pub next_review_date: Option<NaiveDateTime>,
}

/// Represents a new review session entry to be inserted into the database
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Validate)]
pub struct NewReviewSession {
    #[validate(range(min = 1))]
    pub word_id: i32,
    #[validate(range(min = 1, max = 5))]
    pub recall_score: i32,
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_duration",
            deserialize_with = "deserialize_duration"
        )
    )] // Serialize and deserialize as number of seconds
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, FromRow)]
pub struct ForgettingCurve {
    pub curve_id: i32,
    pub word_id: i32, // Foreign key from the words table
    #[cfg_attr(
        feature = "serde",
        serde(
            serialize_with = "serialize_duration",
            deserialize_with = "deserialize_duration"
        )
    )] // Serialize and deserialize as number of seconds
    pub review_interval: Option<Duration>,
    pub retention_rate: f64, // Between 0 and 1
    pub review_count: i32,   // Number of reviews done
}

/// Represents a new forgetting curve entry to be inserted into the database
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Validate)]
pub struct NewForgettingCurve {
    #[validate(range(min = 1))]
    pub word_id: i32,
    #[validate(range(min = 0.0, max = 1.0))]
    pub retention_rate: f64,
    #[validate(range(min = 1))]
    pub review_count: i32,
}

/// Represents pagination parameters for query results
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub size: Option<u32>,
}

#[cfg(feature = "serde")]
fn serialize_duration<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match duration {
        Some(d) => serializer.serialize_i64(d.num_seconds()),
        None => serializer.serialize_none(),
    }
}

#[cfg(feature = "serde")]
fn deserialize_duration<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<i64>::deserialize(deserializer)?;
    Ok(opt.map(Duration::seconds))
}
