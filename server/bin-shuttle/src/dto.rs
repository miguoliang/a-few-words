use engine::types::{MAX_DEFINITION_LENGTH, MAX_PAGE_SIZE, MAX_URL_LENGTH, MAX_WORD_LENGTH};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct NewWord {
    // Assuming a reasonable max length of 100 characters
    #[validate(length(min = 1, max = MAX_WORD_LENGTH))]
    pub word: String,
    // Assuming a reasonable max length of 100 characters
    #[validate(length(min = 1, max = MAX_DEFINITION_LENGTH))]
    pub definition: Option<String>,
    // Assuming a reasonable max length of 100 characters
    #[validate(length(min = 0, max = MAX_URL_LENGTH))]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub definition: Option<String>,
    pub url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

/// Create word from engin::types::Word
impl From<engine::types::Word> for Word {
    fn from(word: engine::types::Word) -> Self {
        Self {
            id: word.word_id,
            word: word.word,
            definition: Some(word.definition),
            url: Some(word.url),
            created_at: word.date_added,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TranslateResponse {
    pub text: String,
}

#[derive(Deserialize, Validate)]
pub struct TranslateParams {
    /// The text to translate
    #[validate(length(min = 0, max = MAX_WORD_LENGTH))]
    pub text: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct ReviewParams {
    #[validate(range(min = 1))] // Assuming a reasonable max length of 100 characters
    pub word_id: i32,
    #[validate(range(min = 1, max = 5))]
    pub recall_score: i32,
}

/// Represents pagination parameters for query results
#[derive(Debug, Clone, Validate, Deserialize)]
pub struct PaginationParams {
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = MAX_PAGE_SIZE))]
    pub size: Option<u64>,
}
