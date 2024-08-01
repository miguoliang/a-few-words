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

pub async fn get_word(id: i32, pool: &PgPool) -> Result<Word> {
    if id < 1 {
        return Err(anyhow!("Invalid ID, must be greater than 0, got {}", id));
    }
    sqlx::query_as("SELECT * FROM words WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| anyhow!(e))
}

pub async fn list_words(offset: Offset, pool: &PgPool) -> Result<Vec<Word>> {
    offset.validate().map_err(|e| anyhow!(e))?;
    sqlx::query_as("SELECT * FROM words")
        .fetch_all(pool)
        .await
        .map_err(|e| anyhow!(e))
}

pub async fn delete_word(id: i32, pool: &PgPool) -> Result<()> {
    if id < 1 {
        return Err(anyhow!("Invalid ID, must be greater than 0, got {}", id));
    }
    sqlx::query("DELETE FROM words WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| anyhow!(e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, thread::sleep};

    use super::*;
    use once_cell::sync::{Lazy, OnceCell};
    use sqlx::postgres::PgPoolOptions;
    use std::time;
    use testcontainers::{
        core::WaitFor, runners::AsyncRunner, ContainerAsync, GenericImage, ImageExt,
    };
    use tokio::sync::Mutex;

    #[derive(Debug)]
    struct TestResources {
        container: Mutex<Option<ContainerAsync<GenericImage>>>,
        pub connection_pool: Arc<PgPool>,
    }

    impl TestResources {
        async fn new() -> Self {
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

            let host = container.get_host().await.unwrap().to_string();
            let port = container.get_host_port_ipv4(5432).await.unwrap();
            let connection_string =
                format!("postgres://username:password@{}:{}/a_few_words", host, port);

            let p = loop {
                if let Ok(pc) = PgPoolOptions::new()
                    .max_connections(1)
                    .connect(&connection_string)
                    .await
                {
                    break pc;
                }
                sleep(time::Duration::from_millis(100));
            };

            sqlx::migrate!()
                .run(&p)
                .await
                .expect("Failed to run migrations");

            Self {
                container: Mutex::new(Some(container)),
                connection_pool: Arc::new(p),
            }
        }

        async fn cleanup(&self) {
            println!("Cleaning up test resources");
            if let Some(container) = self.container.lock().await.take() {
                println!("Stopping and removing container");
                container.stop().await.unwrap();
                container.rm().await.unwrap();
            }
        }
    }

    static TEST_RESOURCES: OnceCell<Arc<TestResources>> = OnceCell::new();
    static TOKIO_RUNTIME: Lazy<tokio::runtime::Runtime> =
        Lazy::new(|| tokio::runtime::Runtime::new().unwrap());

    #[ctor::ctor]
    fn setup() {
        TOKIO_RUNTIME.block_on(async {
            let resources = TestResources::new().await;
            TEST_RESOURCES.set(Arc::new(resources)).unwrap();
        });
    }
    #[ctor::dtor]
    fn teardown() {
        if let Some(resources) = TEST_RESOURCES.get() {
            TOKIO_RUNTIME.block_on(resources.cleanup());
        }
    }

    async fn get_connection_pool() -> Arc<PgPool> {
        Arc::clone(&TEST_RESOURCES.get().unwrap().connection_pool)
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
        let retrieved_word = get_word(word.id, &pool).await.unwrap();
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
}
