use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::request_context::RequestContext;
use super::{Error, ExtractorResult};

// async trait leads to an error here, and it compiles without it so maybe it is not needed
// what this returns is what is can be extracted by the middleware through its parameters
impl <S> FromRequestParts<S> for RequestContext where S: Send + Sync {
    // this does not extract from the body
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> ExtractorResult<Self> {
        let extracted_from_req = parts
        .extensions
        .get::<RequestContext>()
        .ok_or(Error::RequestContextNotInExtensions)?
        .clone();

        Ok(extracted_from_req)
    }
}

