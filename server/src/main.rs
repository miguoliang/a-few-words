use actix_web::{
    error,
    web::{self, Data, ServiceConfig},
    Responder,
};
use diesel::{pg::Pg, r2d2, PgConnection, RunQueryDsl, SelectableHelper};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use models::{NewWord, Word};
use shuttle_actix_web::ShuttleActixWeb;

mod models;
mod schema;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn run_migrations(conn: &mut impl MigrationHarness<Pg>) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run database migrations");
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://username:{secrets.PASSWORD}@localhost:5432/a-few-words"
    )]
    conn_str: String,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let app_config = move |cfg: &mut ServiceConfig| {
        // connect to PostgreSQL database
        let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_str);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("database URL should be valid path to SQLite DB file");
        let mut conn = pool.get().expect("couldn't get DB connection from pool");
        run_migrations(&mut conn);
        drop(conn);
        cfg.app_data(Data::new(pool.clone()))
            .route("/words", web::get().to(get_words))
            .route("/words", web::post().to(create_word));
    };
    Ok(app_config.into())
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
