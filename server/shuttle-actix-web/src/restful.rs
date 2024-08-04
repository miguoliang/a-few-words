use super::cognito;
use crate::cognito::Claims;
use actix_web::{
    delete, error, get, post,
    web::{self, Json, Query},
    Responder, Result,
};
use engine::types::{NewWord, Offset, Word};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub cognito_validator: cognito::CognitoValidator,
}

#[get("/{id}")]
pub async fn retrieve(
    path: web::Path<i32>,
    claims: web::ReqData<Claims>,
    state: web::Data<AppState>,
) -> Result<Json<Word>> {
    let word = engine::api::get_word(path.into_inner(), &claims.username, &state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
    Ok(Json(word))
}

#[post("")]
pub async fn add(
    word_new: web::Json<NewWord>,
    state: web::Data<AppState>,
    claims: web::ReqData<Claims>,
) -> Result<Json<Word>> {
    let new_word = NewWord {
        word: word_new.word.clone(),
        url: word_new.url.clone(),
        username: claims.username.clone(),
    };
    let word = engine::api::create_word(new_word, &state.pool)
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
    let words = engine::api::list_words(&claims.username, query.into_inner(), &state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
    Ok(Json(words))
}

#[delete("/{id}")]
pub async fn delete(
    state: web::Data<AppState>,
    claims: web::ReqData<Claims>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    engine::api::delete_word(path.into_inner(), &claims.username, &state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
    Ok(actix_web::HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::{dev::ServiceRequest, Error, test, App, HttpMessage};
    use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
    use anyhow::Result;
    use engine::setup_database;
    use sqlx::postgres::PgPoolOptions;

    struct MockAppState {
        pool: PgPool,
    }

    async fn get_connection_pool() -> PgPool {
        let connection_string = "postgres://username:password@localhost:5432/a_few_words";
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_string)
            .await
            .unwrap();
        setup_database(&pool).await.unwrap();
        pool
    }

    async fn create_mock_app_state() -> MockAppState {
        MockAppState {
            pool: get_connection_pool().await,
        }
    }

    async fn validator(
        req: ServiceRequest,
        _credentials: BearerAuth,
    ) -> Result<ServiceRequest, (Error, ServiceRequest)> {
        req.extensions_mut().insert(Claims {
            exp: 0,
            username: "test".to_string(),
        });
        Ok(req)
    }

    #[actix_web::test]
    async fn test_create_word() {
        let app = test::init_service(
            App::new()
                .app_data(create_mock_app_state())
                .wrap(HttpAuthentication::bearer(validator))
                .service(add),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .set_json(NewWord {
                word: "test".to_string(),
                url: None,
                username: "test".to_string(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
