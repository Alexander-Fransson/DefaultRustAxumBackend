use axum::extract::Request;
use axum::body::Body;
use axum::middleware::Next;
use axum::response::Response;
use crate::request_context::RequestContext;
use super::Result;

pub async fn mw_implant_request_context(
    mut req: Request<Body>,
    next: Next
) -> Result<Response> {

    let request_context = RequestContext::root();

    req.extensions_mut().insert(request_context);

    Ok(next.run(req).await)
}
