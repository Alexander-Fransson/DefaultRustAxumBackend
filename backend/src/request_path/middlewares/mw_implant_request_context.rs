use std::str::FromStr;
use axum::extract::{Request, State};
use axum::body::Body;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;
use crate::crypt::jwt_token::JwtToken;
use crate::data_access::user_controller::UserController;
use crate::data_access::DataAccessManager;
use crate::request_path::error::{Result, Error};
use crate::request_context::RequestContext;
use crate::views::user::UserForAuth;
use crate::request_path::cookie::{set_jwt_cookie ,AUTH_COOKIE_NAME, delete_jwt_cookie};

pub async fn mw_implant_request_context_if_jwt(
    State(da): State<DataAccessManager>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next
) -> Result<Response> {

    let request_context = request_context_from_jwt_cookie(da, &cookies).await;

    if let Ok(rc) = request_context {
        req.extensions_mut().insert(rc);
    } else if !matches!(request_context, Err(Error::MissingAuthCookie)) {
        delete_jwt_cookie(&cookies)?;
    }

    Ok(next.run(req).await)
}

async fn request_context_from_jwt_cookie(da: DataAccessManager, cookies: &Cookies) -> Result<RequestContext> {
    let auth_token_str = cookies.get(AUTH_COOKIE_NAME)
    .map(|c| c.value().to_string())
    .ok_or(Error::MissingAuthCookie)?;
    
    let auth_token = JwtToken::from_str(&auth_token_str)?;

    let user_for_auth: UserForAuth = UserController::get(&da, auth_token.user_id).await
    .map_err(|e| Error::DataAccess(e.to_string()))?;

    auth_token.validate(&user_for_auth.token_encryption_salt.to_string())
    .map_err(|e| Error::InvalidJwt(e.to_string()))?;

    set_jwt_cookie(&cookies, user_for_auth.id, &user_for_auth.token_encryption_salt.to_string())
    .map_err(|e| Error::FailedToSetJwtCookie(e.to_string()))?;

    RequestContext::new(user_for_auth.id)
    .map_err(|e| Error::RequestContextError(e))
}

// example for tests
pub async fn _mw_implant_request_context(
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next
) -> Result<Response> {

    let auth_cookie = cookies.get(AUTH_COOKIE_NAME)
    .map(|c| c.value().parse::<i64>());

    if let Some(Ok(auth_cookie)) = auth_cookie {
        let request_context = RequestContext::new(auth_cookie)
        .map_err(|e| Error::RequestContextError(e))?;

        req.extensions_mut().insert(request_context);
    };

    Ok(next.run(req).await)
}
