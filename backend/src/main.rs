use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use config_env::get_env_variables;
use request_context::RequestContext;
use request_path::custom_extractors::ExtractorResult;
use request_path::routes::{auth_routes, user_routes};
use log::tracer_config::enable_tracing;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use axum::{middleware, Json, Router};
use axum::routing::get;
use tokio::net::TcpListener;
use uuid::Uuid;
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
    .nest("/auth", auth_routes)
    .nest("/login_required/api/v1", user_routes_requiering_request_context)
    .route("/hello_word", get(|| async {"Hello, World!"}))
    .layer(middleware::map_response(main_response_mapper))
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

async fn main_response_mapper(
    _request_context: ExtractorResult<RequestContext>,
    _uri: Uri,
    _req_method: Method,
    res: Response
) -> Response {
    // hmm it seams like it rejects the request context, look into what that is about
    
    let request_id = Uuid::new_v4();
    let service_error = res.extensions().get::<Error>();

    // println!("RES IS SAYING {:#?}", res);

    println!("SERVICE ERROR WAS SAYING {:#?}", service_error);

    let client_error = service_error.map(|e| e.to_client_error());

    println!("CLIENT ERROR WAS SAYING {:#?}", client_error);

    let err_response = client_error
    .as_ref()
    .map(|(status, error)| {
        let err_body = json!({
            "error": {
                "type": error.as_ref(),
                "request_id": request_id.to_string()
            }
        });
        (*status, Json(err_body)).into_response()
    });

    // log

    //println!("TEHE RESPONSE WAS SAYING {:#?}", err_response);

    err_response.unwrap_or(res)
}