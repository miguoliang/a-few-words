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
        INSERT INTO review_sessions (word_id, next_review_date)
        VALUES ($1, $2)
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
        SELECT EXTRACT(EPOCH FROM (next_review_date - review_date)) / 86400 AS current_interval
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
