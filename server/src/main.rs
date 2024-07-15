use std::io;

use actix_web::{
    error,
    web::{self, Data},
    App, HttpServer, Responder,
};
use diesel::{r2d2, PgConnection, RunQueryDsl, SelectableHelper};
use dotenvy::dotenv;
use models::{NewWord, Word};

mod models;
mod schema;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    // connect to PostgreSQL database
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file");

    // start HTTP server on port 8080
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .route("/words", web::get().to(get_words))
            .route("/words", web::post().to(create_word))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn get_words(pool: Data<DbPool>) -> actix_web::Result<impl Responder> {
    use self::schema::words::dsl::*;
    let results = web::block(move || {
        let mut conn = pool.get().expect("couldn't get DB connection from pool");
        words.load::<Word>(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(actix_web::HttpResponse::Ok().json(results))
}

async fn create_word(
    pool: Data<DbPool>,
    new_word: web::Json<NewWord>,
) -> actix_web::Result<impl Responder> {
    let word = web::block(move || {
        let mut conn = pool.get().expect("couldn't get DB connection from pool");
        diesel::insert_into(schema::words::table)
            .values(new_word.0)
            .returning(Word::as_returning())
            .get_result(&mut conn)
            .expect("couldn't insert new word into DB")
    })
    .await
    .map_err(error::ErrorInternalServerError)?;

    Ok(actix_web::HttpResponse::Ok().json(word))
}
