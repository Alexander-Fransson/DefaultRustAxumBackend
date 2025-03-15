use axum::extract::Request;
use axum::body::Body;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::Cookies;
use crate::gate::error::GateResult;
use crate::request_context::RequestContext;
use super::MiddlewareError;
use crate::gate::AUTH_COOKIE_NAME;

// next step is to test this

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
