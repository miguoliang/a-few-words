use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use sqlx::PgPool;
use tokio::runtime::Runtime;
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

static SETUP: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };

    use super::*;
    use lazy_static::lazy_static;
    use once_cell::sync::Lazy;
    use sqlx::postgres::PgPoolOptions;
    use std::sync::Mutex as StdMutex;
    use testcontainers::{
        core::WaitFor, runners::AsyncRunner, ContainerAsync, GenericImage, ImageExt,
    };
    use tokio::{
        runtime::{Handle, Runtime},
        sync::Mutex,
    };

    async fn initialize_test_runtime() -> (ContainerAsync<GenericImage>, PgPool) {
        let container = GenericImage::new("postgres", "16-alpine")
            .with_wait_for(WaitFor::message_on_stdout(
                "database system is ready to accept connections",
            ))
            .with_env_var("POSTGRES_DB", "a_few_words")
            .with_env_var("POSTGRES_USER", "username")
            .with_env_var("POSTGRES_PASSWORD", "password")
            .start()
            .await
            .unwrap();

        let connection_string = format!(
            "postgres://username:password@{}:{}/a_few_words",
            container.get_host().await.unwrap(),
            container.get_host_port_ipv4(5432).await.unwrap()
        );

        let pool = loop {
            match PgPoolOptions::new()
                .max_connections(5)
                .connect(&connection_string)
                .await
            {
                Ok(pool) => break pool,
                Err(_) => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    continue;
                }
            };
        };

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        (container, pool)
    }

    #[tokio::test]
    async fn test_create_word() {
        let (container, pool) = initialize_test_runtime().await;
        let new_word = NewWord {
            word: "test_create_word".to_string(),
            url: None,
            username: "test".to_string(),
        };
        let word = create_word(new_word, &pool).await.unwrap();
        container.rm().await.unwrap();
        
        assert_eq!(word.word, "test_create_word");
        assert_eq!(word.url, None);
        assert_eq!(word.username, "test");
    }

    #[tokio::test]
    async fn test_get_word() {
        let (container, pool) = initialize_test_runtime().await;
        let new_word = NewWord {
            word: "test_get_word".to_string(),
            url: None,
            username: "test".to_string(),
        };
        let word = create_word(new_word, &pool).await.unwrap();
        let retrieved_word = get_word(word.id, "test", &pool).await.unwrap();
        container.rm().await.unwrap();
        assert_eq!(word, retrieved_word);
    }

    #[tokio::test]
    async fn test_list_words() {
        let (container, pool) = initialize_test_runtime().await;
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
        container.rm().await.unwrap();
        assert!(words.into_iter().any(|w| w.word == "test_list_words"));
    }

    #[tokio::test]
    async fn test_delete_word() {
        let (container, pool) = initialize_test_runtime().await;
        let new_word = NewWord {
            word: "test_delete_word".to_string(),
            url: None,
            username: "test".to_string(),
        };
        let word = create_word(new_word, &pool).await.unwrap();
        delete_word(word.id, "test", &pool).await.unwrap();
        let result = get_word(word.id, "test", &pool).await;
        container.rm().await.unwrap();
        assert!(result.is_err());
    }
}
