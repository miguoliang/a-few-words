use anyhow::{anyhow, Result};
use sqlx::PgPool;
use validator::Validate;

use crate::types::{NewWord, Offset, Word};

pub async fn create_word(new_word: NewWord, pool: &PgPool) -> Result<Word> {
    new_word.validate().map_err(|e| anyhow!(e))?;
    sqlx::query_as("INSERT INTO words(word, url, username) VALUES ($1, $2, $3) RETURNING id, word, url, username, created_at")
        .bind(&new_word.word)
        .bind(&new_word.url)
        .bind(&new_word.username)
        .fetch_one(pool)
        .await
        .map_err(|e| anyhow!(e))
}

pub async fn get_word(id: i32, pool: PgPool) -> Result<Word> {
    if id < 1 {
        return Err(anyhow!("Invalid ID, must be greater than 0, got {}", id));
    }
    sqlx::query_as("SELECT * FROM words WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
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
    use super::*;
    use once_cell::sync::OnceCell;
    use sqlx::postgres::PgPoolOptions;
    use testcontainers::{core::WaitFor, runners::AsyncRunner, GenericImage, ImageExt};

    static CONNECTION_POOL: OnceCell<PgPool> = OnceCell::new();

    #[ctor::ctor]
    fn setup() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let runner = GenericImage::new("postgres", "alpine")
                .with_wait_for(WaitFor::message_on_stdout(
                    "database system is ready to accept connections",
                ))
                .with_env_var("POSTGRES_DB", "a_few_words")
                .with_env_var("POSTGRES_USER", "username")
                .with_env_var("POSTGRES_PASSWORD", "password")
                .start()
                .await
                .unwrap();

            let host = runner.get_host().await.unwrap().to_string();
            let port = runner.get_host_port_ipv4(5432).await.unwrap();
            let connection_string =
                format!("postgres://username:password@{}:{}/a_few_words", host, port);
            let p = PgPoolOptions::new()
                .max_connections(1)
                .connect(&connection_string)
                .await
                .unwrap();
            CONNECTION_POOL.set(p.clone()).unwrap();
            sqlx::migrate!()
                .run(CONNECTION_POOL.get().unwrap())
                .await
                .expect("Failed to run migrations");
        });
    }

    #[tokio::test]
    async fn test_create_word() {
        let pool = CONNECTION_POOL.get().unwrap();
        let new_word = NewWord {
            word: "test".to_string(),
            url: None,
            username: "test".to_string(),
        };
        let word = create_word(new_word, pool).await.unwrap();
        assert_eq!(word.word, "test");
        assert_eq!(word.url, None);
        assert_eq!(word.username, "test");
        delete_word(word.id, pool).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_word() {
        let pool = CONNECTION_POOL.get().unwrap();
        let new_word = NewWord {
            word: "test".to_string(),
            url: None,
            username: "test".to_string(),
        };
        let word = create_word(new_word, pool).await.unwrap();
        let retrieved_word = get_word(word.id, pool.clone()).await.unwrap();
        assert_eq!(word, retrieved_word);
        delete_word(word.id, pool).await.unwrap();
    }

    #[tokio::test]
    async fn test_list_words() {
        let pool = CONNECTION_POOL.get().unwrap();
        let new_word = NewWord {
            word: "test".to_string(),
            url: None,
            username: "test".to_string(),
        };
        create_word(new_word, pool).await.unwrap();
        let words = list_words(
            Offset {
                offset: None,
                size: None,
            },
            pool,
        )
        .await
        .unwrap();
        assert_eq!(words.len(), 1);
        delete_word(words[0].id, pool).await.unwrap();
    }
}
