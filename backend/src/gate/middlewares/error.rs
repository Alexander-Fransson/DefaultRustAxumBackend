use axum::{http::StatusCode, response::{
	IntoResponse,
	Response
}};
use std::num::ParseIntError;
use crate::gate::custom_extractors;
use crate::request_context;


#[derive(Debug, Clone)]
pub enum MiddlewareError {
	ParseError,
	RequestContextError(request_context::Error)
}

impl From<ParseIntError> for MiddlewareError {
    fn from(_err: ParseIntError) -> Self {
        MiddlewareError::ParseError
    }
}

impl From<request_context::Error> for MiddlewareError {
	fn from(err: request_context::Error) -> Self {
		MiddlewareError::RequestContextError(err)
	}
}

impl core::fmt::Display for MiddlewareError {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for MiddlewareError {}