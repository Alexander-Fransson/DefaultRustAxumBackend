use config_env::get_env_variables;
use log::tracer_config::enable_tracing;

mod log;
mod config_env;
mod error;

pub use error::{Error, Result};
use tracing::info;

#[tokio::main]
async fn main() {

    enable_tracing();
    
    let web_folder = &get_env_variables().WEB_FOLDER;
    info!("the web folder got form the env var is: {}", web_folder);
}
