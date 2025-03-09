use axum::extract::Request;
use axum::body::Body;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;
use crate::request_context::RequestContext;
use super::{Result, Error};
use crate::gate::AUTH_COOKIE_NAME;

// next step is to test this

pub async fn mw_implant_request_context(
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next
) -> Result<Response> {

    let auth_cookie = cookies.get(AUTH_COOKIE_NAME)
    .map(|c| c.value().parse::<i64>())
    .transpose()?
    .ok_or(Error::NoAuthCookieFound)?;

    let request_context = RequestContext::new(auth_cookie)?;

    req.extensions_mut().insert(request_context);

    Ok(next.run(req).await)
}
