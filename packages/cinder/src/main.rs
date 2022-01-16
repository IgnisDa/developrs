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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .format_target(true)
        .init();

    // set up database connection pool
    let db_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable");
    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    let state = AppState { conn };

    let bind = std::env::var("BIND")
        .or::<String>(Ok("127.0.0.1:8000".into()))
        .unwrap();

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
