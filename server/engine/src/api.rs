use sqlx::PgPool;
use validator::Validate;

#[cfg(feature = "translate")]
use crate::translate::{translate_text, Language};

use super::error::Error;
use super::types::{NewWord, Offset, Word};

pub async fn create_word(new_word: NewWord, pool: &PgPool) -> Result<Word, Error> {
    new_word.validate()?;
    let word = sqlx::query_as("INSERT INTO words(word, url, username) VALUES ($1, $2, $3) RETURNING id, word, url, username, created_at, updated_at")
        .bind(&new_word.word)
        .bind(&new_word.url)
        .bind(&new_word.username)
        .fetch_one(pool)
        .await?;
    Ok(word)
}

pub async fn get_word(id: i32, username: &str, pool: &PgPool) -> Result<Word, Error> {
    if id < 1 {
        return Err(Error::Validation(validator::ValidationErrors::new()));
    }
    let word = sqlx::query_as("SELECT * FROM words WHERE id = $1 AND username = $2")
        .bind(id)
        .bind(username)
        .fetch_one(pool)
        .await?;
    Ok(word)
}

pub async fn list_words(username: &str, offset: Offset, pool: &PgPool) -> Result<Vec<Word>, Error> {
    offset.validate()?;
    let words = sqlx::query_as(
        "SELECT * FROM words WHERE username = $1 ORDER BY created_at OFFSET $2 LIMIT $3",
    )
    .bind(username)
    .bind(offset.offset.unwrap_or(0))
    .bind(offset.size.unwrap_or(10))
    .fetch_all(pool)
    .await?;
    Ok(words)
}

pub async fn delete_word(id: i32, username: &str, pool: &PgPool) -> Result<(), Error> {
    if id < 1 {
        return Err(Error::Validation(validator::ValidationErrors::new()));
    }
    sqlx::query("DELETE FROM words WHERE id = $1 AND username = $2")
        .bind(id)
        .bind(username)
        .execute(pool)
        .await?;
    Ok(())
}

#[cfg(feature = "translate")]
pub async fn translate_from_english_to_chinese(key: &str, word: &str) -> Result<String, Error> {
    translate_text(key, word, Language::English, Language::Chinese).await
}

#[cfg(test)]
pub mod tests {

    use crate::setup_database;

    use super::*;
    use sqlx::postgres::PgPoolOptions;

    pub async fn get_connection_pool() -> PgPool {
        let connection_string = "postgres://username:password@localhost:5432/a_few_words";
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await
            .unwrap();
        setup_database(&pool).await.unwrap();
        pool
    }

    #[cfg(feature = "translate")]
    #[tokio::test]
    async fn test_translate_from_english_to_chinese() {
        dotenv::dotenv().ok();
        let word = "hello";
        let key = std::env::var("GOOGLE_TRANSLATE_API_KEY").unwrap();
        let translation = translate_from_english_to_chinese(&key, word).await.unwrap();
        assert_eq!(translation, "你好");
    }

    #[tokio::test]
    async fn test_create_word() {
        let pool = get_connection_pool().await;
        let new_word = NewWord {
            word: "test_create_word".to_string(),
            url: None,
            username: "test".to_string(),
        };
        let word = create_word(new_word, &pool).await.unwrap();
        assert_eq!(word.word, "test_create_word");
        assert_eq!(word.url, None);
        assert_eq!(word.username, "test");
    }

    #[tokio::test]
    async fn test_get_word() {
        let pool = get_connection_pool().await;
        let new_word = NewWord {
            word: "test_get_word".to_string(),
            url: None,
            username: "test".to_string(),
        };
        let word = create_word(new_word, &pool).await.unwrap();
        let retrieved_word = get_word(word.id, "test", &pool).await.unwrap();
        assert_eq!(word, retrieved_word);
    }

    #[tokio::test]
    async fn test_list_words() {
        let pool = get_connection_pool().await;
        let new_word = NewWord {
            word: "test_list_words".to_string(),
            url: None,
            username: "test".to_string(),
        };
        create_word(new_word, &pool).await.unwrap();
        let words = list_words(
            "test",
            Offset {
                offset: None,
                size: None,
            },
            &pool,
        )
        .await
        .unwrap();
        assert!(words.into_iter().any(|w| w.word == "test_list_words"));
    }

    #[tokio::test]
    async fn test_delete_word() {
        let pool = get_connection_pool().await;
        let new_word = NewWord {
            word: "test_delete_word".to_string(),
            url: None,
            username: "test".to_string(),
        };
        let word = create_word(new_word, &pool).await.unwrap();
        delete_word(word.id, "test", &pool).await.unwrap();
        let result = get_word(word.id, "test", &pool).await;
        assert!(result.is_err());
    }
}
