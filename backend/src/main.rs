use config_env::get_env_variables;
use db_setup::{make_migrations, reset_db, create_app_user_connection_pool};
use log::tracer_config::enable_tracing;
use tracing::info;

mod log;
mod config_env;
mod error;
mod utils;
mod db_setup;

pub use error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {

    enable_tracing();
    
    //need to create an error that incorporates the sqlx error

    let pool = create_app_user_connection_pool().await?;
    
    reset_db().await?;
    make_migrations(&pool).await?;
    
    let web_folder = &get_env_variables().WEB_FOLDER;
    let db_connection = &get_env_variables().DB_CONNECTION_STRING;
    info!("the web folder got form the env var is: {}", web_folder);
    info!("the db connection string got form the env var is: {}", db_connection);

    Ok(())
}
