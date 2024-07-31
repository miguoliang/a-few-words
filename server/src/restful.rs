use super::cognito;
use crate::{
    cognito::Claims,
    dto::{Offset, Word, WordNew},
};
use actix_web::{
    error, get, post, put,
    web::{self, Json, Query},
    Result,
};
use sqlx::PgPool;
use validator::Validate;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub cognito_validator: cognito::CognitoValidator,
}

#[get("/{id}")]
pub async fn retrieve(path: web::Path<i32>, state: web::Data<AppState>) -> Result<Json<Word>> {
    let word = sqlx::query_as("SELECT * FROM words WHERE id = $1")
        .bind(*path)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(word))
}

#[post("")]
pub async fn add(
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

#[get("")]
pub async fn list(
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

#[put("/{id}")]
pub async fn update(
    state: web::Data<AppState>,
    claims: web::ReqData<Claims>,
    body: Json<WordNew>,
    path: web::Path<i32>,
) -> Result<Json<Word>> {
    if body.validate().is_err() {
        return Err(error::ErrorBadRequest("Invalid request body".to_string()));
    }

    let word = sqlx::query_as("UPDATE words SET word = $1, url = $2 WHERE id = $3 AND username = $4 RETURNING id, word, url, username, created_at")
        .bind(&body.word)
        .bind(&body.url)
        .bind(*path)
        .bind(&claims.username)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;

    Ok(Json(word))
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::{http::StatusCode, test, App};

    #[actix_web::test]
    async fn test_create_word() {
        let app = test::init_service(App::new().service(add)).await;
        let req = test::TestRequest::post()
            .uri("/words")
            .set_json(WordNew {
                word: "test".to_string(),
                url: None,
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
