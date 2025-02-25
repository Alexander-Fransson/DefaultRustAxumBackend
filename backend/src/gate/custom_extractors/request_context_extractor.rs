use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::request_context::RequestContext;
use super::{Error, Result};

// async trait leads to an error here, and it compiles without it so maybe it is not needed
impl <S> FromRequestParts<S> for RequestContext where S: Send + Sync {
    // this does not extract from the body
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        parts
        .extensions
        .get::<Result<RequestContext>>()
        .ok_or(Error::RequestContextNotInExtensions)?
        .clone()
    }
}

