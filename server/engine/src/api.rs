use anyhow::{anyhow, Result};
use sqlx::PgPool;
use validator::Validate;

use crate::types::{NewWord, Offset, Word};

pub async fn create_word(new_word: NewWord, pool: &PgPool) -> Result<Word> {
    new_word.validate().map_err(|e| anyhow!(e))?;
    sqlx::query_as("INSERT INTO words(word, url, username) VALUES ($1, $2, $3) RETURNING id, word, url, username, created_at, updated_at")
        .bind(&new_word.word)
        .bind(&new_word.url)
        .bind(&new_word.username)
        .fetch_one(pool)
        .await
        .map_err(|e| anyhow!(e))
}

pub async fn get_word(id: i32, username: &str, pool: &PgPool) -> Result<Word> {
    if id < 1 {
        return Err(anyhow!("Invalid ID, must be greater than 0, got {}", id));
    }
    sqlx::query_as("SELECT * FROM words WHERE id = $1 AND username = $2")
        .bind(id)
        .bind(username)
        .fetch_one(pool)
        .await
        .map_err(|e| anyhow!(e))
}

pub async fn list_words(username: &str, offset: Offset, pool: &PgPool) -> Result<Vec<Word>> {
    offset.validate().map_err(|e| anyhow!(e))?;
    sqlx::query_as("SELECT * FROM words WHERE username = $1 ORDER BY created_at OFFSET $2 LIMIT $3")
        .bind(username)
        .bind(offset.offset.unwrap_or(0))
        .bind(offset.size.unwrap_or(10))
        .fetch_all(pool)
        .await
        .map_err(|e| anyhow!(e))
}

pub async fn delete_word(id: i32, username: &str, pool: &PgPool) -> Result<()> {
    if id < 1 {
        return Err(anyhow!("Invalid ID, must be greater than 0, got {}", id));
    }
    sqlx::query("DELETE FROM words WHERE id = $1 AND username = $2")
        .bind(id)
        .bind(username)
        .execute(pool)
        .await
        .map_err(|e| anyhow!(e))?;
    Ok(())
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
            .connect(&connection_string)
            .await
            .unwrap();
        setup_database(&pool).await.unwrap();
        pool       
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
