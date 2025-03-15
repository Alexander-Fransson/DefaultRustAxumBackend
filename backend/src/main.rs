use config_env::get_env_variables;
use gate::routes::user_routes;
use log::tracer_config::enable_tracing;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use axum::{middleware, Router};
use axum::routing::get;
use tokio::net::TcpListener;
use crate::data_access::DataAccessManager;
use gate::middlewares::{
    mw_implant_request_context,
    mw_require_request_context
};

mod log;
mod config_env;
mod error;
mod utils;
pub mod data_access;
mod views;
mod gate;
mod integration_tests;
mod request_context;
mod crypt;

pub use error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {

    enable_tracing();
    serve_server().await?;
    
    Ok(())
}

async fn serve_server() -> Result<()> {
            
    let data_access_manager = DataAccessManager::new().await?;
    let user_routes = user_routes(data_access_manager)
    //.route_layer(middleware::from_fn(mw_require_request_context))
    ; // now it just has to be tested and documented, remember that the impl response had to be done in the gate error

    let main_router = Router::new()
    .nest("/api/v1", user_routes)
    .route("/hello_word", get(|| async {"Hello, World!"}))
    .layer(middleware::from_fn(mw_implant_request_context))
    .layer(CookieManagerLayer::new())
    ;

    let url = get_env_variables().LISTENER_URL;

    let listener = TcpListener::bind(url).await?;

    info!("listening on: http://{}", url);

    axum::serve(listener, main_router).await?;
    
    Ok(())
}