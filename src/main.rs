use actix_web::{web, App, HttpServer};
use actix_web::web::Data;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
mod user;

async fn _connect() -> Result<sqlx::PgPool, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = _connect().await.expect("Failed to connect to the database.");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .service(web::scope("/app")
                .route("/users", web::get().to(user::list))
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
