use std::sync::Arc;

use actix_web::{
    dev::ServiceRequest,
    middleware::Logger,
    web::{self, Data, ServiceConfig},
    HttpMessage,
};
use actix_web_httpauth::{
    extractors::{bearer::BearerAuth, AuthenticationError},
    headers::www_authenticate::bearer::Bearer,
    middleware::HttpAuthentication,
};
use engine::setup_database;
use restful::{add, delete, list, retrieve, translate, AppState};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;
use tokio::sync::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod cognito;
mod dto;
mod error;
mod restful;

#[derive(OpenApi)]
#[openapi(
    info(
        version = "1.0.0",
        title = "A Few Words API",
        description = "A RESTful API for managing words"
    ),
    paths(
        restful::retrieve,
        restful::add,
        restful::list,
        restful::delete,
        restful::translate
    ),
    components(schemas(dto::NewWord, dto::Word, dto::TranslateResponse))
)]
struct ApiDoc;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://username:password@localhost:5432/a_few_words"
    )]
    pool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let google_translate_api_key = secrets
        .get("google_translate_api_key")
        .expect("google translate api key was not found");

    let cognito_region = secrets
        .get("cognito_region")
        .expect("cognito region was not found");

    let cognito_user_pool_id = secrets
        .get("cognito_user_pool_id")
        .expect("cognito user pool id was not found");

    let cognito_client_id = secrets
        .get("cognito_client_id")
        .expect("cognito client id was not found");

    setup_database(&pool)
        .await
        .expect("Failed to setup database");

    let cognito_validator = Arc::new(Mutex::new(
        cognito::CognitoValidator::new(&cognito_region, &cognito_user_pool_id, &cognito_client_id)
            .await
            .expect("Failed to create Cognito validator"),
    ));

    tokio::spawn(scheduled_update_jwk(cognito_validator.clone()));

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .service(
            web::scope("/api/v1")
                .wrap(Logger::default())
                .wrap(HttpAuthentication::bearer(validator))
                .service(retrieve)
                .service(add)
                .service(list)
                .service(delete)
                .service(translate)
                .app_data(Data::new(AppState {
                    pool: Arc::new(pool),
                    cognito_validator: Some(cognito_validator),
                    google_translate_api_key: google_translate_api_key.clone(),
                })),
        );
    };

    Ok(config.into())
}

async fn scheduled_update_jwk(cognito_validator: Arc<Mutex<cognito::CognitoValidator>>) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600 * 2)).await;
        cognito_validator.lock().await.update_jwk().await.unwrap();
    }
}

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let cognito_validator = req
        .app_data::<Data<AppState>>()
        .unwrap()
        .cognito_validator
        .clone()
        .unwrap();
    let cognito_validator = cognito_validator.lock().await;
    let token = credentials.token();
    match cognito_validator.validate_token(token) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => {
            let ae = AuthenticationError::new(Bearer::default());
            Err((actix_web::Error::from(ae), req))
        }
    }
}

#[cfg(test)]
pub mod test_utils {
    use serde::Deserialize;
    use tokio::fs;

    #[derive(Debug, Deserialize)]
    pub struct Secrets {
        pub google_translate_api_key: String,
        pub cognito_user_pool_id: String,
        pub cognito_client_id: String,
        pub cognito_region: String,
    }

    pub async fn get_secrets() -> Secrets {
        let toml_str = fs::read_to_string("Secrets.toml").await.unwrap();
        toml::from_str(&toml_str).unwrap()
    }
}
