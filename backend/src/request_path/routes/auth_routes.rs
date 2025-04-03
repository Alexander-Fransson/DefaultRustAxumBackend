use serde_json::{json, Value};
use tower_cookies::Cookies;
use crate::{request_path::cookie::set_jwt_cookie, views::user::UserForRegister};
use crate::views::user::UserForLogin;
use crate::data_access:: DataAccessManager;
use crate::request_path::Result;
use crate::request_path::cookie::delete_jwt_cookie;
use crate::request_path::Error;
use axum::{
    Router,
    Json,
    extract::State,
    routing::post,
};
use crate::data_access::user_controller::UserController;

fn generate_success_json() -> Json<Value> {
    Json(json!({"resuult":{
        "success": true
    }}))    
}

// next steps are to remove the cookie from the response somethime

pub fn auth_routes(da: DataAccessManager) -> Router {
    Router::new()
    .route("/login", post(login_handler))
    .route("/logout", post(logout_handler))
    .route("/register", post(register_handler))
    .with_state(da)
}

async fn logout_handler(cookies: Cookies) -> Result<Json<Value>> {
    delete_jwt_cookie(&cookies)?;
    Ok(generate_success_json())
}

async fn login_handler(
    cookies: Cookies,
    State(da): State<DataAccessManager>, 
    Json(credentials): Json<UserForLogin>
) -> Result<Json<Value>> {
    let token_parts = UserController::login_user(&da, credentials).await
    .map_err(|e| Error::DataAccess(e))?;

    set_jwt_cookie(&cookies, token_parts.id, &token_parts.token_encryption_salt.to_string())?;

    Ok(generate_success_json())
}

async fn register_handler(
    cookies: Cookies,
    State(da): State<DataAccessManager>,
    Json(user): Json<UserForRegister>
) -> Result<Json<Value>> {

    let token_parts = UserController::register_user(&da, user).await
    .map_err(|e| {
        println!("{}", e);
        Error::DataAccess(e)
    })?;

    set_jwt_cookie(&cookies, token_parts.id, &token_parts.token_encryption_salt.to_string())?;
    
    Ok(generate_success_json())
}