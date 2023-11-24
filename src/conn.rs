use actix_web::{HttpResponse};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn connection() -> HttpResponse {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
    let _pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

        HttpResponse::Ok().body("Connected!")
}