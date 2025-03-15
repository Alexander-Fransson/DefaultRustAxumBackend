use axum::extract::Request;
use axum::body::Body;
use axum::middleware::Next;
use axum::response::Response;
use crate::gate::error::GateResult;
use crate::request_context::RequestContext;
use crate::gate::custom_extractors::ExtractorResult;

pub async fn mw_require_request_context(
    request_context: ExtractorResult<RequestContext>,
    req: Request<Body>,
    next: Next
) -> GateResult<Response> {

    request_context?;

    Ok(next.run(req).await)
}

// the first parameter cannot be used, hmmm