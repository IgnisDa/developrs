use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;
use sea_orm::DatabaseConnection;

mod data;
mod entities;
mod errors;
mod routes;

#[macro_use]
extern crate log;

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

const DATABASE_URL: &str = "DATABASE_URL";
const SERVER_HOST: &str = "SERVER_HOST";
const MODE: &str = "MODE";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .format_target(true)
        .init();

    // set up database connection pool
    let db_url = std::env::var(DATABASE_URL)
        .unwrap_or_else(|_| panic!("{} environment variable not set", DATABASE_URL));
    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    let state = AppState { conn };

    if std::env::var(MODE).unwrap_or_else(|_| "development".to_string()) == "production" {
        let pool = sqlx_core::postgres::PgPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();
        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Unable to run migrations");
        pool.close().await;
    }

    let bind =
        std::env::var(SERVER_HOST).unwrap_or_else(|_| "127.0.0.1:8000".to_string());

    println!("Starting server at: {:?}...", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/{app_name}")
                    .service(routes::update_app)
                    .service(routes::find_app_by_name),
            )
            .service(routes::get_all_apps)
    })
    .bind(&bind)?
    .run()
    .await
}
