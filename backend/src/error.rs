use std::fmt::Debug;
use crate::data_access;
use crate::request_path;
use crate::utils;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use strum_macros::AsRefStr;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ErrorForClient {
	LOGIN_FAILED,
	ENTITY_NOT_FOUND,
	FAILED_TO_AUTHENTICATE,
	
	INVALID_REQUEST_PARAMETERS,
	INTERNAL_ERROR
}

#[derive(Debug)]
pub enum Error {
    CannotFindEnvWithSuchName(&'static str),
    FailedToParse(&'static str),
	DataAccessError(data_access::Error),
	RequestPathError(request_path::Error),
	Io(std::io::Error),
	Utils(utils::Error)
}

impl Error {
	pub fn to_client_error(&self) -> (StatusCode,ErrorForClient) {
		match self {
			Self::DataAccessError(data_access::Error::IncorrectPassword) 
			=> (StatusCode::UNAUTHORIZED, ErrorForClient::LOGIN_FAILED),

			Self::DataAccessError(data_access::Error::EntityNotFound) 
			=> (StatusCode::NOT_FOUND, ErrorForClient::ENTITY_NOT_FOUND),

			Self::RequestPathError(request_path::Error::MissingAuthCookie) |
			Self::RequestPathError(request_path::Error::InvalidJwt(_)) |
			Self::RequestPathError(request_path::Error::FailedToSetJwtCookie(_)) |
			Self::RequestPathError(request_path::Error::RequestContextError(_)) |
			Self::RequestPathError(request_path::Error::Extractor(_)) 
			=> (StatusCode::UNAUTHORIZED, ErrorForClient::FAILED_TO_AUTHENTICATE),
			
			// how do I track invalid parameters?

			_ => (StatusCode::INTERNAL_SERVER_ERROR, ErrorForClient::INTERNAL_ERROR)
		}
	}
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		// magic is to happen in map response middleware ServerError -> ClientError 

		let mut response = StatusCode::NOT_FOUND.into_response();
		response.extensions_mut().insert(self.to_string());

		response
	}
}

impl From<utils::Error> for Error {
	fn from(err: utils::Error) -> Self {
		Self::Utils(err)
	}
}

impl From<request_path::Error> for Error {
	fn from(err: request_path::Error) -> Self {
		Self::RequestPathError(err)
	}
}

impl From<data_access::Error> for Error {
	fn from(err: data_access::Error) -> Self {
		Self::DataAccessError(err)
	}
}

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Self::Io(err)
	}
}

impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}