use config_env::get_env_variables;
use request_path::routes::{auth_routes, user_routes};
use log::tracer_config::enable_tracing;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use axum::{middleware, Router};
use axum::routing::get;
use tokio::net::TcpListener;
use crate::data_access::DataAccessManager;
use request_path::middlewares::{
    mw_implant_request_context_if_jwt, mw_require_request_context,
    //mw_require_request_context
};

mod log;
mod config_env;
mod error;
mod utils;
pub mod data_access;
mod views;
mod request_path;
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
    let plane_user_routes = user_routes(data_access_manager.clone());
    let user_routes_requiering_request_context = user_routes(data_access_manager.clone())
    .layer(middleware::from_fn(mw_require_request_context)); 

    let auth_routes = auth_routes(data_access_manager.clone());

    let main_router = Router::new()
    .nest("/api/v1", plane_user_routes)
    .nest("/api/v1/auth", auth_routes)
    .nest("login_required/api/v1", user_routes_requiering_request_context)
    .route("/hello_word", get(|| async {"Hello, World!"}))
    .layer(middleware::from_fn_with_state(
        data_access_manager.clone(),
        mw_implant_request_context_if_jwt
    ))
    .layer(CookieManagerLayer::new())
    ;

    let url = get_env_variables().LISTENER_URL;

    let listener = TcpListener::bind(url).await?;

    info!("listening on: http://{}", url);

    axum::serve(listener, main_router).await?;
    
    Ok(())
}