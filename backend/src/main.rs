use db_setup::{
    make_migrations, 
    reset_db, 
    create_serive_user_connection_pool
};
use log::tracer_config::enable_tracing;
use tracing::info;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;

mod log;
mod config_env;
mod error;
mod utils;
mod db_setup;
mod data_access;
mod authentication;

pub use error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {

    enable_tracing();
    
    let pool = create_serive_user_connection_pool().await?;
    
    reset_db().await?;
    make_migrations(&pool).await?;

    let main_router = Router::new()
    .route("/hello_word", get(|| async {"Hello, World!"})); 

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    info!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, main_router).await?;
    
    Ok(())
}
