use std::sync::Arc;

use super::cognito;
use super::cognito::Claims;
use super::dto::{
    NewWord, PaginationParams, ReviewParams, TranslateParams, TranslateResponse, Word,
};
use actix_web::{
    delete, get, post,
    web::{self},
    Responder, Result,
};
use sqlx::PgPool;
use tokio::sync::Mutex;

use super::error::IntoActixError;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<PgPool>,
    pub cognito_validator: Option<Arc<Mutex<cognito::CognitoValidator>>>,
    pub google_translate_api_key: String,
}

/// Retrieve a word by ID
#[utoipa::path(
    responses(
        (status = 200, description = "Word retrieved successfully", body = Word),
        (status = 404, description = "Word not found"),
        (status = 403, description = "Word does not belong to user"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Authorization" = ["Bearer"])
    ),
    params(
        ("id" = i32, description = "The ID of the word to retrieve"),
        ("Authorization" = String, description = "Bearer token")
    )
)]
#[get("/words/{id}")]
pub async fn retrieve(
    path: web::Path<i32>,
    claims: web::ReqData<Claims>,
    state: web::Data<AppState>,
) -> Result<web::Json<Word>> {
    let word = engine::api::get_word(path.into_inner(), &claims.username, &state.pool)
        .await
        .map_err(engine::error::Error::into_actix_error)?;
    Ok(web::Json(word.into()))
}

/// Add a new word
#[utoipa::path(
    request_body = NewWord,
    responses(
        (status = 200, description = "Word added successfully", body = inline(Word)),
        (status = 400, description = "Invalid request body"),
        (status = 409, description = "Word already exists"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Authorization" = ["Bearer"])
    ),
    params(
        ("Authorization" = String, description = "Bearer token")
    )
)]
#[post("/words")]
pub async fn add(
    word_new: web::Json<NewWord>,
    state: web::Data<AppState>,
    claims: web::ReqData<Claims>,
) -> Result<web::Json<Word>> {
    let new_word = engine::types::NewWord {
        word: word_new.word.clone(),
        definition: word_new.definition.clone().unwrap_or_default(),
        url: word_new.url.clone().unwrap_or_default(),
        user_id: claims.username.clone(),
        initial_forgetting_rate: Some(0.5),
    };
    let word = engine::api::insert_word(new_word, &state.pool)
        .await
        .map_err(engine::error::Error::into_actix_error)?;
    Ok(web::Json(word.into()))
}

/// Retrieve a list of words
#[utoipa::path(
    responses(
        (status = 200, description = "Words retrieved successfully", body = [Word]),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Authorization" = ["Bearer"])
    ),
    params(
        ("Authorization" = String, Header, description = "Bearer token"),
        ("page" = u32, Query, description = "The page number to retrieve, starting from 0", example = 0),
        ("size" = u32, Query, description = "The number of words per page, max 100", example = 10)
    )
)]
#[get("/words")]
pub async fn list(
    state: web::Data<AppState>,
    claims: web::ReqData<Claims>,
    query: web::Query<PaginationParams>,
) -> Result<web::Json<Vec<Word>>> {
    let words = engine::api::get_words(&claims.username, query.page, query.size, &state.pool)
        .await
        .map_err(engine::error::Error::into_actix_error)?;
    Ok(web::Json(words.into_iter().map(|w| w.into()).collect()))
}

/// Delete a word by ID
#[utoipa::path(
    responses(
        (status = 204, description = "Word deleted successfully"),
        (status = 403, description = "Word does not belong to user"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Authorization" = ["Bearer"])
    ),
    params(
        ("Authorization" = String, description = "Bearer token"),
        ("word_id" = i32, description = "The ID of the word to delete")
    )
)]
#[delete("/words/{word_id}")]
pub async fn delete(
    state: web::Data<AppState>,
    claims: web::ReqData<Claims>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    engine::api::delete_word(path.into_inner(), &claims.username, &state.pool)
        .await
        .map_err(engine::error::Error::into_actix_error)?;
    Ok(actix_web::HttpResponse::NoContent().finish())
}

/// Translate text
#[utoipa::path(
    responses(
        (status = 200, description = "Translated text retrieved successfully", body = TranslateResponse),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Authorization" = ["Bearer"])
    ),
    params(
        ("Authorization" = String, description = "Bearer token"),
        ("text" = String, Query, description = "The text to translate")
    )
)]
#[get("/translate")]
pub async fn translate(
    state: web::Data<AppState>,
    query: web::Query<TranslateParams>,
) -> Result<web::Json<TranslateResponse>> {
    let text = query.text.clone();
    let translated_text = engine::translate::translate_text(
        &state.google_translate_api_key,
        &text,
        engine::translate::Language::English,
        engine::translate::Language::Chinese,
    )
    .await
    .map_err(engine::error::Error::into_actix_error)?;
    Ok(web::Json(TranslateResponse {
        text: translated_text,
    }))
}

#[utoipa::path(
    request_body = ReviewParams,
    responses(
        (status = 204, description = "Review updated successfully"),
        (status = 400, description = "Invalid request body"),
        (status = 403, description = "Word does not belong to user"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("Authorization" = ["Bearer"])
    ),
    params(
        ("Authorization" = String, description = "Bearer token"),
        ("word_id" = i32, description = "The ID of the word to review"),
        ("recall_score" = u32, description = "The recall score for the word")
    )
)]
#[post("/review")]
pub async fn review(
    state: web::Data<AppState>,
    claims: web::ReqData<Claims>,
    body: web::Json<ReviewParams>,
) -> Result<impl Responder> {
    let word_belongs_to_user =
        engine::api::check_word_belongs_to_user(body.word_id, &claims.username, &state.pool)
            .await
            .map_err(engine::error::Error::into_actix_error)?;
    if !word_belongs_to_user {
        return Err(actix_web::error::ErrorForbidden(
            "Word does not belong to user",
        ));
    }

    // Validate recall_score before updating
    if body.recall_score < 1 || body.recall_score > 5 {
        return Err(actix_web::error::ErrorBadRequest(
            "Invalid recall score. Must be between 1 and 5.",
        ));
    }

    engine::api::update_next_review_date(body.word_id, body.recall_score, &state.pool)
        .await
        .map_err(engine::error::Error::into_actix_error)?;
    Ok(actix_web::HttpResponse::NoContent().finish())
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::{dev::ServiceRequest, test, App, Error, HttpMessage};
    use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
    use engine::setup_database;
    use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
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
            google_translate_api_key: "test".to_string(),
        }
    }

    async fn validator(
        req: ServiceRequest,
        _credentials: BearerAuth,
    ) -> Result<ServiceRequest, (Error, ServiceRequest)> {
        req.extensions_mut().insert(Claims {
            exp: 0,
            username: "test_user".to_string(),
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
                definition: Some("Nostrud voluptate ea do sunt qui elit sunt velit ullamco aliqua reprehenderit consequat.".to_string()),
                url: Some("http://localhost:8080".to_string()),
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
                definition: Some("Nostrud voluptate ea do sunt qui elit sunt velit ullamco aliqua reprehenderit consequat.".to_string()),
                url: Some("http://localhost:8080".to_string()),
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
                definition: Some("Dolor pariatur enim dolor labore labore Lorem duis officia tempor ipsum tempor nulla mollit nisi.".to_string()),
                url: Some("http://localhost:8080".to_string()),
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
                definition: Some("Dolor pariatur enim dolor labore labore Lorem duis officia tempor ipsum tempor nulla mollit nisi.".to_string()),
                url: Some("http://localhost:8080".to_string()),
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

    #[actix_web::test]
    async fn test_translate_api() {
        let toml = crate::test_utils::get_secrets().await;

        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState {
                    pool: Arc::new(get_connection_pool().await),
                    cognito_validator: None,
                    google_translate_api_key: toml.google_translate_api_key.clone(),
                }))
                .wrap(HttpAuthentication::bearer(validator))
                .service(translate),
        )
        .await;

        let encoded_text = utf8_percent_encode("have a good time", NON_ALPHANUMERIC).to_string();
        let req = test::TestRequest::get()
            .uri(format!("/translate?text={encoded_text}").as_str())
            .insert_header(("Authorization", "Bearer test"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(
            resp.status().is_success(),
            "Response Status Code: {:?}",
            resp.status()
        );
        let resp: TranslateResponse = test::read_body_json(resp).await;
        assert_eq!(resp.text, "玩的很开心");
    }

    #[actix_web::test]
    async fn test_review_api() {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(create_mock_app_state().await))
                .wrap(HttpAuthentication::bearer(validator))
                .service(review)
                .service(add),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/words")
            .set_json(NewWord {
                word: "test_review_api".to_string(),
                definition: Some("Dolor pariatur enim dolor labore labore Lorem duis officia tempor ipsum tempor nulla mollit nisi.".to_string()),
                url: Some("http://localhost:8080".to_string()),
            })
            .insert_header(("Authorization", "Bearer test"))
            .to_request();
        let word: Word = test::call_and_read_body_json(&app, req).await;

        let req = test::TestRequest::post()
            .uri("/review")
            .set_json(ReviewParams {
                word_id: word.id,
                recall_score: 5,
            })
            .insert_header(("Authorization", "Bearer test"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(
            resp.status().is_success(),
            "Response Status Code: {:?}",
            resp.status()
        );

        let to_review = engine::api::get_words_for_review(
            "test_user",
            Some(0),
            Some(10),
            &get_connection_pool().await,
        )
        .await
        .unwrap();

        assert!(!to_review.is_empty());
        assert!(to_review.iter().any(|w| w.word_id == word.id));
    }
}
