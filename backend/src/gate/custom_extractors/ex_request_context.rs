use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::request_context::RequestContext;
use super::{Error, ExtractorResult};

// what this returns is what is can be extracted by the middleware through its parameters
impl <S> FromRequestParts<S> for RequestContext where S: Send + Sync {
    // this does not extract from the body
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> ExtractorResult<Self> {
        let extracted_from_req = parts
        .extensions
        .get::<RequestContext>()
        .ok_or(Error::RequestContextNotInRequestExtensions)?
        .clone();

        Ok(extracted_from_req)
    }
}

