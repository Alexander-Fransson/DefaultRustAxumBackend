use tracing::info;
use tracing_subscriber::EnvFilter;

pub fn enable_tracing() {

    tracing_subscriber::fmt()
    
    // time is quite verbose so it may be removed durring development
    // it can be formatted but it is quite difficult and not woth it right now
    .without_time()
    .with_file(true)
    .with_line_number(true)
    
    // removes the crate name from the logs
    .with_target(false) 
    
    // reads the RUST_LOG env var
    .with_env_filter(EnvFilter::from_default_env())
    
    // makes tracing macros like info! work
    .init();

    info!("Tracing enabled!");
}