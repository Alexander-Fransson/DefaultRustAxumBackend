use config_env::get_env_variables;
use gate::routes::user_routes;
use log::tracer_config::enable_tracing;
use tracing::info;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
use crate::data_access::DataAccessManager;

mod log;
mod config_env;
mod error;
mod utils;
pub mod data_access;
mod views;
mod gate;
mod integration_tests;
mod request_context;

pub use error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {

    enable_tracing();
    serve_server().await?;
    
    Ok(())
}

async fn serve_server() -> Result<()> {
            
    let data_access_manager = DataAccessManager::new().await?;
    let user_routes = user_routes(data_access_manager); // now it just has to be tested and documented, remember that the impl response had to be done in the gate error

    let main_router = Router::new()
    .nest("/api/v1", user_routes)
    .route("/hello_word", get(|| async {"Hello, World!"}));

    let url = get_env_variables().LISTENER_URL;

    let listener = TcpListener::bind(url).await?;

    info!("listening on: http://{}", url);

    axum::serve(listener, main_router).await?;
    
    Ok(())
}