use actix_web::{
    error::{self},
    get,
    middleware::Logger,
    post,
    web::{self, Data, Json, Query, ServiceConfig},
    Error, HttpMessage, Result,
};
use actix_web_httpauth::{
    extractors::AuthenticationError, headers::www_authenticate::bearer::Bearer,
    middleware::HttpAuthentication,
};
use anyhow::Context;
use cognito::Claims;
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::{types::chrono, FromRow, PgPool};
use validator::Validate;

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
async fn add(
    word_new: web::Json<WordNew>,
    state: web::Data<AppState>,
    claims: web::ReqData<Claims>,
) -> Result<Json<Word>> {
    let word = sqlx::query_as("INSERT INTO words(word, url, username) VALUES ($1, $2, $3) RETURNING id, word, url, username, created_at")
        .bind(&word_new.word)
        .bind(&word_new.url)
        .bind(&claims.username)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(word))
}

#[derive(Deserialize, Validate)]
struct Offset {
    #[validate(range(min = 0))]
    offset: Option<i32>,
    #[validate(range(min = 1, max = 100))]
    size: Option<i32>,
}

#[get("")]
async fn list(
    state: web::Data<AppState>,
    claims: web::ReqData<Claims>,
    query: Query<Offset>,
) -> Result<Json<Vec<Word>>> {
    if query.validate().is_err() {
        return Err(error::ErrorBadRequest(
            "Invalid query parameters".to_string(),
        ));
    }

    let bind = sqlx::query_as(
        "SELECT * FROM words WHERE username = $1 ORDER BY created_at DESC OFFSET $2 LIMIT $3",
    )
    .bind(&claims.username)
    .bind(query.offset.unwrap_or(0))
    .bind(query.size.unwrap_or(10));
    let words = bind
        .fetch_all(&state.pool)
        .await
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    Ok(Json(words))
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    cognito_validator: cognito::CognitoValidator,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://username:password@localhost:5432/a_few_words"
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
            .app_data::<Data<AppState>>()
            .unwrap()
            .cognito_validator
            .verify_token(token)
        {
            Ok(token_data) => {
                req.extensions_mut().insert(token_data.claims);
                Ok(req)
            }
            Err(_) => {
                let ae = AuthenticationError::new(Bearer::default());
                Err((Error::from(ae), req))
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
                .service(list)
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
    pub username: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}
