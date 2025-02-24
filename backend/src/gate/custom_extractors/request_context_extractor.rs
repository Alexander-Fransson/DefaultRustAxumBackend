use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use async_trait::async_trait;

use crate::request_context::RequestContext;

use super::{Error, Result};



#[async_trait]
impl <S: Send + Sync> FromRequestParts<S> for RequestContext {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        parts
        .extensions
        .get::<Result<RequestContext>>()
        .ok_or(Error::RequestContextNotInExtensions)?
        .clone()
    }
}

// need to rethink error handling on this floor maybe have one master and many lesser