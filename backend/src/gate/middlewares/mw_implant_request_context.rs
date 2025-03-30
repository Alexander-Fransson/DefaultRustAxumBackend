use std::str::FromStr;
use axum::extract::{Request, State};
use axum::body::Body;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;
use crate::crypt::jwt_token::JwtToken;
use crate::data_access::user_controller::UserController;
use crate::data_access::DataAccessManager;
use crate::gate::error::GateResult;
use crate::request_context::RequestContext;
use crate::views::user::UserForAuth;
use super::MiddlewareError;
use crate::gate::cookie::{set_jwt_cookie ,AUTH_COOKIE_NAME, delete_jwt_cookie};
use crate::gate::GateError;

// the extra error stuff is pretty tedipus maybe rename gate to request and move all the error to the gate error
// also test this function

pub async fn mw_implant_request_context_if_jwt(
    State(da): State<DataAccessManager>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next
) -> GateResult<Response> {

    let request_context = request_context_from_jwt_cookie(da, &cookies).await;

    if request_context.is_err() && 
        !matches!(request_context, Err(GateError::Middleware(MiddlewareError::MissingAuthCookie))) {
        delete_jwt_cookie(&cookies)?;   
    }

    req.extensions_mut().insert(request_context?);

    Ok(next.run(req).await)
}

async fn request_context_from_jwt_cookie(da: DataAccessManager, cookies: &Cookies) -> GateResult<RequestContext> {
    let auth_token_str = cookies.get(AUTH_COOKIE_NAME)
    .map(|c| c.value().to_string())
    .ok_or(MiddlewareError::MissingAuthCookie)?;
    
    let auth_token = JwtToken::from_str(&auth_token_str)?;

    let user_for_auth: UserForAuth = UserController::get(&da, auth_token.user_id).await
    .map_err(|e| MiddlewareError::DataAccess(e.to_string()))?;

    auth_token.validate(&user_for_auth.token_encryption_salt)
    .map_err(|e| MiddlewareError::InvalidJwt(e.to_string()))?;

    set_jwt_cookie(&cookies, user_for_auth.id, &user_for_auth.token_encryption_salt)
    .map_err(|e| MiddlewareError::FailedToSetJwtCookie(e.to_string()))?;

    RequestContext::new(user_for_auth.id)
    .map_err(|e| GateError::Middleware(MiddlewareError::RequestContextError(e)))
}

// example for tests
pub async fn mw_implant_request_context(
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next
) -> GateResult<Response> {

    let auth_cookie = cookies.get(AUTH_COOKIE_NAME)
    .map(|c| c.value().parse::<i64>());

    if let Some(Ok(auth_cookie)) = auth_cookie {
        let request_context = RequestContext::new(auth_cookie)
        .map_err(|e| MiddlewareError::RequestContextError(e))?;

        req.extensions_mut().insert(request_context);
    };

    Ok(next.run(req).await)
}
