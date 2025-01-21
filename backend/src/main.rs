use config_env::get_env_variables;
use log::tracer_config::enable_tracing;
use tracing::info;
use utils::_dev::reset_db;

mod log;
mod config_env;
mod error;
mod utils;

pub use error::{Error, Result};

#[tokio::main]
async fn main() {

    enable_tracing();
    
    let web_folder = &get_env_variables().WEB_FOLDER;
    let db_connection = &get_env_variables().DB_CONNECTION_STRING;
    info!("the web folder got form the env var is: {}", web_folder);
    info!("the db connection string got form the env var is: {}", db_connection);

    reset_db().await.unwrap();
}
