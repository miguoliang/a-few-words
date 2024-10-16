use engine::types::{MAX_DEFINITION_LENGTH, MAX_PAGE_SIZE, MAX_URL_LENGTH, MAX_WORD_LENGTH};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;
use utoipa::ToSchema;
use validator::Validate;

/// New word
///
/// # Example
/// ```json
/// {
///     "word": "hello",
///     "definition": "a greeting",
///     "url": "https://example.com"
/// }
/// ```
#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct NewWord {
    #[validate(length(min = 1, max = MAX_WORD_LENGTH))]
    #[schema(example = "hello")]
    pub word: String,
    #[validate(length(min = 1, max = MAX_DEFINITION_LENGTH))]
    #[schema(example = "a greeting")]
    pub definition: Option<String>,
    #[validate(length(min = 0, max = MAX_URL_LENGTH))]
    #[schema(example = "https://example.com")]
    pub url: Option<String>,
}

/// Word
///
/// # Example
/// ```json
/// {
///     "id": 1,
///     "word": "hello",
///     "definition": "a greeting",
///     "url": "https://example.com",
///     "created_at": "2024-01-01T00:00:00Z"
/// }
/// ```
#[derive(Serialize, Deserialize, ToSchema)]
pub struct Word {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = "hello")]
    pub word: String,
    #[schema(example = "a greeting")]
    pub definition: Option<String>,
    #[schema(example = "https://example.com")]
    pub url: Option<String>,
    #[schema(example = "2024-01-01T00:00:00Z", value_type = String)]
    pub created_at: chrono::NaiveDateTime,
}

/// Create word from engine::types::Word
///
/// # Example
/// ```json
/// {
///     "id": 1,
///     "word": "hello",
///     "definition": "a greeting",
///     "url": "https://example.com",
///     "created_at": "2024-01-01T00:00:00Z"
/// }
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

/// Translate response
///
/// # Example
/// ```json
/// {
///     "text": "Hello, world!"
/// }
/// ```
#[derive(Serialize, Deserialize, ToSchema)]
pub struct TranslateResponse {
    #[schema(example = "Hello, world!")]
    pub text: String,
}

/// Translate params
///
/// # Example
/// ```json
/// {
///     "text": "Hello, world!"
/// }
/// ```
#[derive(Deserialize, Validate)]
pub struct TranslateParams {
    /// The text to translate
    #[validate(length(min = 0, max = MAX_WORD_LENGTH))]
    pub text: String,
}

/// Review params
///
/// # Example
/// ```json
/// {
///     "word_id": 1,
///     "recall_score": 3
/// }
/// ```
#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct ReviewParams {
    #[validate(range(min = 1))]
    #[schema(example = 1)]
    pub word_id: i32,
    #[validate(range(min = 1, max = 5))]
    #[schema(example = 3)]
    pub recall_score: i32,
}

/// Pagination params
#[derive(Debug, Clone, Validate, Deserialize)]
pub struct PaginationParams {
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = MAX_PAGE_SIZE))]
    pub size: Option<u64>,
}
