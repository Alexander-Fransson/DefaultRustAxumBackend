use axum::{
    http::StatusCode,
    response::{
        IntoResponse,
        Response
    }
};
use super::{
    middlewares,
    custom_extractors,
    routes
};
use crate::crypt;

pub type GateResult<T> = core::result::Result<T, GateError>;

#[derive(Debug)]
pub enum GateError {
    Middleware(middlewares::MiddlewareError),
    Extractor(custom_extractors::Error),
    Route(routes::Error),
    Crypt(crypt::Error)
}

impl From<crypt::Error> for GateError {
    fn from(error: crypt::Error) -> Self {
        GateError::Crypt(error)
    }
}

impl From<middlewares::MiddlewareError> for GateError {
    fn from(error: middlewares::MiddlewareError) -> Self {
        GateError::Middleware(error)
    }
}

impl From<custom_extractors::Error> for GateError {
    fn from(error: custom_extractors::Error) -> Self {
        GateError::Extractor(error)
    }
}

impl From<routes::Error> for GateError {
    fn from(error: routes::Error) -> Self {
        GateError::Route(error)
    }
}   

impl IntoResponse for GateError {
	fn into_response(self) -> Response {
		// magic is to happen in map response middleware ServerError -> ClientError 

		let mut response = StatusCode::NOT_FOUND.into_response();
		response.extensions_mut().insert(self.to_string());

		response
	}
}

impl core::fmt::Display for GateError {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for GateError {}