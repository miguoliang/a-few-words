use serde::{Deserialize, Serialize};
use sqlx::{types::chrono, FromRow};
use validator::Validate;

#[derive(Deserialize)]
pub struct WordNew {
    pub word: String,
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
