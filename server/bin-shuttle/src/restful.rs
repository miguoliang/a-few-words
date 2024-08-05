use std::{rc::Rc, sync::Arc};

use super::cognito;
use super::cognito::Claims;
use actix_web::{
    delete, get, post,
    web::{self, Json, Query},
    Responder, Result,
};
use engine::types::{Offset, Word};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::error::IntoActixError;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<PgPool>,
    pub cognito_validator: Option<Rc<cognito::CognitoValidator>>,
}

#[get("/words/{id}")]
pub async fn retrieve(
    path: web::Path<i32>,
    claims: web::ReqData<Claims>,
    state: web::Data<AppState>,
) -> Result<Json<Word>> {
    let word = engine::api::get_word(path.into_inner(), &claims.username, &state.pool)
        .await
        .map_err(engine::types::Error::into_actix_error)?;
    Ok(Json(word))
}

#[derive(Serialize, Deserialize)]
pub struct NewWord {
    word: String,
    url: Option<String>,
}

#[post("/words")]
pub async fn add(
    word_new: web::Json<NewWord>,
    state: web::Data<AppState>,
    claims: web::ReqData<Claims>,
) -> Result<Json<Word>> {
    let new_word = engine::types::NewWord {
        word: word_new.word.clone(),
        url: word_new.url.clone(),
        username: claims.username.clone(),
    };
    let word = engine::api::create_word(new_word, &state.pool)
        .await
        .map_err(engine::types::Error::into_actix_error)?;
    Ok(Json(word))
}

#[get("/words")]
pub async fn list(
    state: web::Data<AppState>,
    claims: web::ReqData<Claims>,
    query: Query<Offset>,
) -> Result<Json<Vec<Word>>> {
    let words = engine::api::list_words(&claims.username, query.into_inner(), &state.pool)
        .await
        .map_err(engine::types::Error::into_actix_error)?;
    Ok(Json(words))
}

#[delete("/words/{id}")]
pub async fn delete(
    state: web::Data<AppState>,
    claims: web::ReqData<Claims>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    engine::api::delete_word(path.into_inner(), &claims.username, &state.pool)
        .await
        .map_err(engine::types::Error::into_actix_error)?;
    Ok(actix_web::HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::{dev::ServiceRequest, test, App, Error, HttpMessage};
    use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
    use engine::setup_database;
    use sqlx::postgres::PgPoolOptions;
    use web::Data;

    async fn get_connection_pool() -> PgPool {
        let connection_string = "postgres://username:password@localhost:5432/a_few_words";
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await
            .unwrap();
        setup_database(&pool).await.unwrap();
        pool
    }

    async fn create_mock_app_state() -> AppState {
        AppState {
            pool: Arc::new(get_connection_pool().await),
            cognito_validator: None,
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
    async fn test_create_word_api() {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(create_mock_app_state().await))
                .wrap(HttpAuthentication::bearer(validator))
                .service(add),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/words")
            .set_json(NewWord {
                word: "test_create_word_api".to_string(),
                url: None,
            })
            .insert_header(("Authorization", "Bearer test"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(
            resp.status().is_success(),
            "Response Status Code: {:?}",
            resp.status()
        );
    }

    #[actix_web::test]
    async fn test_retrieve_word_api() {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(create_mock_app_state().await))
                .wrap(HttpAuthentication::bearer(validator))
                .service(retrieve)
                .service(add),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/words")
            .set_json(NewWord {
                word: "test_retrieve_word_api".to_string(),
                url: None,
            })
            .insert_header(("Authorization", "Bearer test"))
            .to_request();
        let word: Word = test::call_and_read_body_json(&app, req).await;

        let req = test::TestRequest::get()
            .uri(format!("/words/{}", word.id).as_str())
            .insert_header(("Authorization", "Bearer test"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(
            resp.status().is_success(),
            "Response Status Code: {:?}",
            resp.status()
        );
    }

    #[actix_web::test]
    async fn test_list_words_api() {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(create_mock_app_state().await))
                .wrap(HttpAuthentication::bearer(validator))
                .service(list)
                .service(add),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/words")
            .set_json(NewWord {
                word: "test_list_words_api".to_string(),
                url: None,
            })
            .insert_header(("Authorization", "Bearer test"))
            .to_request();
        test::call_service(&app, req).await;

        let req = test::TestRequest::get()
            .uri("/words")
            .insert_header(("Authorization", "Bearer test"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(
            resp.status().is_success(),
            "Response Status Code: {:?}",
            resp.status()
        );
        let resp: Vec<Word> = test::read_body_json(resp).await;
        assert!(!resp.is_empty(), "No words returned");
        assert!(
            resp.iter().any(|w| w.word == "test_list_words_api"),
            "Word not found"
        );
    }

    #[actix_web::test]
    async fn test_delete_word_api() {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(create_mock_app_state().await))
                .wrap(HttpAuthentication::bearer(validator))
                .service(delete)
                .service(retrieve)
                .service(add),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/words")
            .set_json(NewWord {
                word: "test_delete_word_api".to_string(),
                url: None,
            })
            .insert_header(("Authorization", "Bearer test"))
            .to_request();
        let word: Word = test::call_and_read_body_json(&app, req).await;

        let req = test::TestRequest::delete()
            .uri(format!("/words/{}", word.id).as_str())
            .insert_header(("Authorization", "Bearer test"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(
            resp.status().is_success(),
            "Response Status Code: {:?}",
            resp.status()
        );

        let req = test::TestRequest::get()
            .uri(format!("/words/{}", word.id).as_str())
            .insert_header(("Authorization", "Bearer test"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status().as_u16(),
            404,
            "Response Status Code: {:?}",
            resp.status()
        );
    }
}