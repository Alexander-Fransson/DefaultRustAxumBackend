use axum::{
    http::StatusCode,
    response::{
        IntoResponse,
        Response
    }
};
use super::custom_extractors;
use crate::{crypt, request_context};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    //middleware
    MissingAuthCookie,
    InvalidJwt(String),
    FailedToSetJwtCookie(String),
    RequestContextError(request_context::Error),

    Extractor(custom_extractors::Error),

    // other modules
    Crypt(crypt::Error),
    DataAccess(String),
}

impl From<crypt::Error> for Error {
    fn from(error: crypt::Error) -> Self {
        Error::Crypt(error)
    }
}

impl From<custom_extractors::Error> for Error {
    fn from(error: custom_extractors::Error) -> Self {
        Error::Extractor(error)
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

impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}