use tower_cookies::{cookie, Cookies};
use crate::{gate::cookie::set_jwt_cookie, views::user::UserForRegister};
use crate::views::user::UserForLogin;
use crate::data_access:: DataAccessManager;
use super::super::GateResult;
use crate::gate::cookie::delete_jwt_cookie;
use super::Error;
use axum::{
    Router,
    Json,
    extract::State,
    routing::post,
};
use crate::data_access::user_controller::UserController;

// next steps are to remove the cookie from the response somethime

pub fn auth_routes(da: DataAccessManager) -> Router {
    Router::new()
    .route("/auth/login", post(login_handler))
    .route("/auth/logout", post(logout_handler))
    .route("/auth/register", post(register_handler))
    .with_state(da)
}

async fn logout_handler(cookies: Cookies) -> GateResult<()> {
    delete_jwt_cookie(&cookies)?;
    Ok(())
}

async fn login_handler(
    cookies: Cookies,
    State(da): State<DataAccessManager>, 
    Json(credentials): Json<UserForLogin>
) -> GateResult<()> {
    let token_parts = UserController::login_user(&da, credentials).await
    .map_err(|e| Error::DataAccess(e))?;

    set_jwt_cookie(&cookies, token_parts.id, &token_parts.token_encryption_salt)?;

    Ok(())
}

async fn register_handler(
    cookies: Cookies,
    State(da): State<DataAccessManager>,
    Json(user): Json<UserForRegister>
) -> GateResult<()> {
    let token_parts = UserController::register_user(&da, user).await
    .map_err(|e| Error::DataAccess(e))?;

    set_jwt_cookie(&cookies, token_parts.id, &token_parts.token_encryption_salt)?;
    
    Ok(())
}