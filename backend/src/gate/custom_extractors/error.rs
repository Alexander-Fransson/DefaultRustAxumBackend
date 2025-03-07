use axum::{http::StatusCode, response::{
	IntoResponse,
	Response
}};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    RequestContextNotInExtensions
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		// magic is to happen in map response middleware ServerError -> ClientError 

		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
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