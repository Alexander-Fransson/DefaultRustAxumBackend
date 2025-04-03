use axum::{
    http::StatusCode,
    response::{
        IntoResponse,
        Response
    }
};
use super::custom_extractors;
use crate::{crypt, data_access, request_context};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    //middleware
    MissingAuthCookie,
    InvalidJwt(String),
    FailedToSetJwtCookie(String),
    RequestContextError(request_context::Error),

    Extractor(custom_extractors::Error),

    // other modules
    Crypt(String),
    DataAccess(data_access::Error),
}

impl From<crypt::Error> for Error {
    fn from(error: crypt::Error) -> Self {
        Error::Crypt(error.to_string())
    }
}

impl From<custom_extractors::Error> for Error {
    fn from(error: custom_extractors::Error) -> Self {
        Error::Extractor(error)
    }
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
        let main_error = crate::error::Error::RequestPathError(self);

		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
		response.extensions_mut().insert(main_error);

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