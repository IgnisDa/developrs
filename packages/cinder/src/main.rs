use actix_web::{middleware, web, App, HttpServer};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use env_logger::Env;

mod actions;
mod api_errors;
mod db;
mod models;
mod routes;
mod schema;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .format_target(true)
        .init();

    // set up database connection pool
    let connection_string =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable");
    let manager = ConnectionManager::<PgConnection>::new(connection_string);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = std::env::var("BIND")
        .or::<String>(Ok("127.0.0.1:8000".into()))
        .unwrap();

    println!("Starting server at: {:?}...", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/apps")
                    .service(routes::create_app)
                    .service(routes::get_all_apps)
                    .service(routes::find_app_by_id),
            )
    })
    .bind(&bind)?
    .run()
    .await
}
