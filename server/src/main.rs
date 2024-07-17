use actix_web::{
    error, get,
    middleware::Logger,
    post,
    web::{self, Json, ServiceConfig},
    Result,
};
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::{FromRow, PgPool};

#[get("/{id}")]
async fn retrieve(path: web::Path<i32>, state: web::Data<AppState>) -> Result<Json<Word>> {
    let word = sqlx::query_as("SELECT * FROM words WHERE id = $1")
        .bind(*path)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(word))
}

#[post("")]
async fn add(word_new: web::Json<WordNew>, state: web::Data<AppState>) -> Result<Json<Word>> {
    let word = sqlx::query_as("INSERT INTO words(word, url) VALUES ($1, $2) RETURNING id, word, url")
        .bind(&word_new.word)
        .bind(&word_new.url)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(word))
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://username:password@localhost:5432/a-few-words"
    )]
    pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = web::Data::new(AppState { pool });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/words")
                .wrap(Logger::default())
                .service(retrieve)
                .service(add)
                .app_data(state),
        );
    };

    Ok(config.into())
}

#[derive(Deserialize)]
struct WordNew {
    pub word: String,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
struct Word {
    pub id: i32,
    pub word: String,
    pub url: Option<String>,
}