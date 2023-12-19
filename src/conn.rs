use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn _connect() -> Result<sqlx::PgPool, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
