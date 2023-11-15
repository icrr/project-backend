use dotenv::dotenv;
use sqlx::{Pool, Postgres};
pub async fn connect() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = Pool::<Postgres>::connect(&database_url).await?;

    Ok(pool)
}