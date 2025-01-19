use log::tracer_config::enable_tracing;

mod log;

#[tokio::main]
async fn main() {

    enable_tracing();
    
}
