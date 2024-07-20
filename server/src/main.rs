use actix_web::{
    error::{self},
    get,
    middleware::Logger,
    post,
    web::{self, Json, ServiceConfig},
    Error, Result,
};
use actix_web_httpauth::{
    extractors::AuthenticationError, headers::www_authenticate::bearer::Bearer,
    middleware::HttpAuthentication,
};
use anyhow::Context;
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::{types::chrono, FromRow, PgPool};

mod cognito;

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
    let word = sqlx::query_as("INSERT INTO words(word, url, username) VALUES ($1, $2, $3) RETURNING id, word, url, username, created_at")
        .bind(&word_new.word)
        .bind(&word_new.url)
        .bind(&word_new.username)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(word))
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    cognito_validator: cognito::CognitoValidator,
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

    let cognito_validator = cognito::CognitoValidator::new(
        "us-east-1",
        "us-east-1_Qbzi9lvVB",
        "5p99s5nl7nha5tfnpik3r0rb7j",
    )
    .await
    .context("Failed to create Cognito validator")?;

    let state = web::Data::new(AppState {
        pool,
        cognito_validator,
    });

    let auth = HttpAuthentication::bearer(|req, credentials| async move {
        let token = credentials.token();
        match req
            .app_data::<AppState>()
            .unwrap()
            .cognito_validator
            .verify_token(token)
        {
            Ok(_) => return Ok(req),
            Err(_) => {
                let ae = AuthenticationError::new(Bearer::default());
                return Err((Error::from(ae), req));
            }
        }
    });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/words")
                .wrap(Logger::default())
                .wrap(auth)
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
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
struct Word {
    pub id: i32,
    pub word: String,
    pub url: Option<String>,
    pub username: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}
