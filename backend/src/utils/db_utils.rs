use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub async fn create_connection_pool(connection_string: &str) -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let pool =PgPoolOptions::new()
    .max_connections(1)
    .acquire_timeout(Duration::from_secs(5))
    .connect(connection_string)
    .await?;

    Ok(pool)
}