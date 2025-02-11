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
mod data_shapes;
mod gate;

pub use error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {

    enable_tracing();
        
    let data_access_manager = DataAccessManager::new().await?;
    let user_routes = user_routes(data_access_manager); // now it just has to be tested and documented, remember that the impl response had to be done in the gate error

    let main_router = Router::new()
    .nest("/api/v1", user_routes)
    .route("/hello_word", get(|| async {"Hello, World!"}));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    info!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, main_router).await?;
    
    Ok(())
}
