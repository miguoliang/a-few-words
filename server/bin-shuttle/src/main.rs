use std::{rc::Rc, sync::Arc};

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
use restful::{add, delete, list, retrieve, AppState};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;

mod cognito;
mod restful;
mod error;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://username:password@localhost:5432/a_few_words"
    )]
    pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    setup_database(&pool)
        .await
        .expect("Failed to setup database");

    let cognito_validator = cognito::CognitoValidator::new(
        "us-east-1",
        "us-east-1_Qbzi9lvVB",
        "5p99s5nl7nha5tfnpik3r0rb7j",
    )
    .await
    .expect("Failed to create Cognito validator");

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/api/v1")
                .wrap(Logger::default())
                .wrap(HttpAuthentication::bearer(validator))
                .service(retrieve)
                .service(add)
                .service(list)
                .service(delete)
                .app_data(Data::new(AppState {
                    pool: Arc::new(pool),
                    cognito_validator: Some(Rc::new(cognito_validator)),
                })),
        );
    };

    Ok(config.into())
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
