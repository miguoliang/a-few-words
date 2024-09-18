use crate::error::Error;
use crate::types::{NewWord, PaginationParams, Word};
use chrono::Utc;
use sqlx::PgPool;

/// Inserts a new word into the database
///
/// # Arguments
///
/// * `new_word` - The new word to be inserted
/// * `pool` - The database connection pool
///
/// # Returns
///
/// Returns the inserted `Word` if successful, or an `Error` if the operation fails
pub async fn insert_word(new_word: NewWord, pool: &PgPool) -> Result<Word, Error> {
    let now = Utc::now().naive_utc();
    let initial_forgetting_rate = new_word.initial_forgetting_rate.unwrap_or(0.5);

    let word: Word = sqlx::query_as(
        r#"
        INSERT INTO words (user_id, word, definition, url, date_added, initial_forgetting_rate)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING word_id, user_id, word, definition, url, date_added, initial_forgetting_rate
        "#,
    )
    .bind(new_word.user_id)
    .bind(new_word.word)
    .bind(new_word.definition)
    .bind(new_word.url)
    .bind(now)
    .bind(initial_forgetting_rate)
    .fetch_one(pool)
    .await?;

    // Insert into review_sessions
    sqlx::query(
        r#"
        INSERT INTO review_sessions (word_id, review_date, next_review_date)
        VALUES ($1, $2, $2)
        "#,
    )
    .bind(word.word_id)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(word)
}

/// Retrieves a word by its ID and user ID
///
/// # Arguments
///
/// * `word_id` - The ID of the word to retrieve
/// * `user_id` - The ID of the user who owns the word
/// * `pool` - The database connection pool
///
/// # Returns
///
/// Returns a `Word` if successful, or an `Error` if the operation fails
pub async fn get_word(word_id: i32, user_id: &str, pool: &PgPool) -> Result<Word, Error> {
    let word = sqlx::query_as::<_, Word>(
        r#"
        SELECT word_id, user_id, word, definition, url, date_added, initial_forgetting_rate
        FROM words
        WHERE word_id = $1 AND user_id = $2
        "#,
    )
    .bind(word_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(word)
}

/// Retrieves words for a user with pagination
///
/// # Arguments
///
/// * `user_id` - The ID of the user who owns the words
/// * `pagination` - Pagination parameters
/// * `pool` - The database connection pool
///
/// # Returns
///
/// Returns a `Vec<Word>` containing the paginated words, or an `Error` if the operation fails
pub async fn get_words(
    user_id: &str,
    pagination: PaginationParams,
    pool: &PgPool,
) -> Result<Vec<Word>, Error> {
    let words = sqlx::query_as(
        r#"
        SELECT word_id, user_id, word, definition, url, date_added, initial_forgetting_rate
        FROM words
        WHERE user_id = $1
        ORDER BY date_added DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(pagination.size.unwrap_or(10) as i64)
    .bind((pagination.page.unwrap_or(0) * pagination.size.unwrap_or(10)) as i64)
    .fetch_all(pool)
    .await?;

    Ok(words)
}

/// Retrieves words for review with pagination
///
/// # Arguments
///
/// * `user_id` - The ID of the user who owns the words
/// * `pagination` - Pagination parameters
/// * `pool` - The database connection pool
///
/// # Returns
///
/// Returns a `Vec<WordForReview>` if successful, or an `Error` if the operation fails
pub async fn get_words_for_review(
    user_id: &str,
    pagination: &PaginationParams,
    pool: &PgPool,
) -> Result<Vec<Word>, Error> {
    let words = sqlx::query_as::<_, Word>(
        r#"
        SELECT word_id, user_id, word, definition, url, date_added, initial_forgetting_rate, next_review_date
        FROM words
        INNER JOIN review_sessions USING (word_id)
        WHERE user_id = $1 AND next_review_date <= NOW()
        ORDER BY next_review_date ASC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(pagination.size.unwrap_or(10) as i64)
    .bind((pagination.page.unwrap_or(0) * pagination.size.unwrap_or(10)) as i64)
    .fetch_all(pool)
    .await?;

    Ok(words)
}

/// Updates the next review date for a word
///
/// # Arguments
///
/// * `word_id` - The ID of the word to update
/// * `next_review_date` - The new next review date
/// * `pool` - The database connection pool
///
/// # Returns
///
/// Returns `true` if the update is successful, or `false` if the operation fails
pub async fn update_next_review_date(
    word_id: i32,
    recall_score: i32,
    pool: &PgPool,
) -> Result<(), Error> {
    let current_interval: f64 = sqlx::query_scalar(
        r#"
        SELECT (EXTRACT(EPOCH FROM (next_review_date - review_date)) / 86400)::FLOAT8 AS current_interval
        FROM review_sessions
        WHERE word_id = $1
        ORDER BY review_date DESC
        LIMIT 1
        "#,
    )
    .bind(word_id)
    .fetch_one(pool)
    .await?;

    // Determine factor based on recall score
    let factor = match recall_score {
        5 => 2.5,
        4 => 2.0,
        3 => 1.0,
        2 => 0.5,
        1 => 0.25,
        _ => 1.0,
    };

    // Calculate the new interval
    let next_interval = current_interval * factor;

    // Update the next review date
    sqlx::query(
        r#"
        UPDATE review_sessions
        SET next_review_date = CURRENT_TIMESTAMP + INTERVAL '1 day' * $1
        WHERE word_id = $2
        "#,
    )
    .bind(next_interval)
    .bind(word_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Deletes a word by its ID and user ID
///
/// # Arguments
///
/// * `word_id` - The ID of the word to delete
/// * `user_id` - The ID of the user who owns the word
/// * `pool` - The database connection pool
///
/// # Returns
///
/// Returns `true` if the deletion is successful, or `false` if the operation fails
pub async fn delete_word(word_id: i32, user_id: &str, pool: &PgPool) -> Result<(), Error> {
    sqlx::query(
        r#"
        DELETE FROM words
        WHERE word_id = $1 AND user_id = $2
        "#,
    )
    .bind(word_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Check whether a word belongs to a user
///
/// # Arguments
///
/// * `word_id` - The ID of the word to check
/// * `user_id` - The ID of the user to check
/// * `pool` - The database connection pool
///
/// # Returns
///
/// Returns `true` if the word belongs to the user, or `false` if the operation fails
pub async fn check_word_belongs_to_user(
    word_id: i32,
    user_id: &str,
    pool: &PgPool,
) -> Result<bool, Error> {
    let belongs_to_user = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM words
            WHERE word_id = $1 AND user_id = $2
        )
        "#,
    )
    .bind(word_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(belongs_to_user)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_check_word_belongs_to_user() {
        let pool = PgPool::connect("postgres://username:password@localhost:5432/a_few_words")
            .await
            .unwrap();
        let word_id = 1;
        let user_id = "test_user";

        let belongs_to_user = check_word_belongs_to_user(word_id, user_id, &pool)
            .await
            .unwrap();
        assert_eq!(belongs_to_user, false);
    }

    #[tokio::test]
    async fn test_update_next_review_date() {
        let pool = PgPool::connect("postgres://username:password@localhost:5432/a_few_words")
            .await
            .unwrap();
        let recall_score = 5;

        let time_of_insertion = chrono::Utc::now().naive_utc();
        let word = insert_word(
            NewWord {
                word: "test_word".to_string(),
                definition: "test_definition".to_string(),
                url: "test_url".to_string(),
                initial_forgetting_rate: Some(0.5),
                user_id: "test_user".to_string(),
            },
            &pool,
        )
        .await
        .unwrap();

        let word_id = word.word_id;
        update_next_review_date(word_id, recall_score, &pool)
            .await
            .unwrap();
        let next_review_date = sqlx::query_scalar::<_, chrono::NaiveDateTime>(
            r#"
            SELECT next_review_date
            FROM review_sessions
            WHERE word_id = $1
            "#,
        )
        .bind(word_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        assert!(next_review_date > time_of_insertion);
    }
}
