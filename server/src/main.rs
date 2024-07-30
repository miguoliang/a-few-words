use actix_web::{
    middleware::Logger,
    web::{self, Data, ServiceConfig},
    Error, HttpMessage,
};
use actix_web_httpauth::{
    extractors::AuthenticationError, headers::www_authenticate::bearer::Bearer,
    middleware::HttpAuthentication,
};
use anyhow::Context;
use restful::{add, list, retrieve, AppState};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;

mod cognito;
mod dto;
mod restful;

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

    let state = Data::new(AppState {
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
