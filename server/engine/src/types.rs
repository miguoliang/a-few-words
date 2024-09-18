use chrono::{Duration, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a word entry in the database
#[derive(Debug, FromRow, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug)]
pub struct NewReviewSession {
    pub word_id: i32,
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
#[derive(Debug)]
pub struct NewForgettingCurve {
    pub word_id: i32,
    pub retention_rate: f64,
    pub review_count: i32,
}

/// Represents pagination parameters for query results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub size: Option<u32>,
}
