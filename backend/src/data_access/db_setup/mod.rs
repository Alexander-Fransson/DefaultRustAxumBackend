mod db_setup_test;
pub mod error;

use sqlx::postgres::PgPoolOptions;
use tracing::info;
use std::time::Duration;
use sqlx::{Pool,Postgres};
use crate::config_env::get_env_variables;
use std::fs;

pub use error::{Error, Result};

// TODO: include these errors in the main error module
pub async fn _reset_db() -> Result<()> {
    let connect_as_default_user = &get_env_variables().DB_DEFAULT_USER_CONNECTION_STRING;
    let pool = create_connection_pool(connect_as_default_user).await?;

    let file_data  = fs::read_to_string("./db/sql/reset_db/00_recreate_db.sql")
    .map_err(|e| Error::FailedToReadFiles(format!("{}", e)))?;
    
    let recreation_commands: Vec<&str> = file_data.split(";").collect();

    for command in recreation_commands { 
        sqlx::query(command)
        .execute(&pool)
        .await?;
    }

    info!("reset db");

    Ok(())
}

pub async fn make_migrations(pool: &Pool<Postgres>) -> Result<()> {
    
    sqlx::migrate!("./db/sql/migrations")
    .run(pool)
    .await?;

    info!("created migrations");

    Ok(())
}

pub async fn create_connection_pool(connection_string: &str) -> Result<Pool<Postgres>> {
    let pool =PgPoolOptions::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(5))
    .connect(connection_string)
    .await?;

    info!("created connection pool");

    Ok(pool)
}

pub async fn create_serive_user_connection_pool() -> Result<Pool<Postgres>> {
    let connect_as_service_user = &get_env_variables().DB_CONNECTION_STRING;
    let pool = create_connection_pool(connect_as_service_user).await?;

    Ok(pool)
}
