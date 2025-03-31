use axum::extract::Request;
use axum::body::Body;
use axum::middleware::Next;
use axum::response::Response;
use crate::request_path::error::Result;
use crate::request_context::RequestContext;
use crate::request_path::custom_extractors::ExtractorResult;

pub async fn mw_require_request_context(
    request_context: ExtractorResult<RequestContext>,
    req: Request<Body>,
    next: Next
) -> Result<Response> {

    request_context?;

    Ok(next.run(req).await)
}

// the first parameter cannot be used, hmmm