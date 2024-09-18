use chrono::{Duration, NaiveDateTime};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::FromRow;

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
#[derive(Debug)]
pub struct NewWord {
    pub user_id: String,
    pub word: String,
    pub definition: String,
    pub url: String,
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
#[derive(Debug)]
pub struct NewReviewSession {
    pub word_id: i32,
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
#[derive(Debug)]
pub struct NewForgettingCurve {
    pub word_id: i32,
    pub retention_rate: f64,
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
