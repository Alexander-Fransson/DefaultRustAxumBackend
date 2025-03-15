use std::fmt::Debug;
use axum::{http::StatusCode, response::{
	IntoResponse,
	Response
}};

use crate::data_access;

#[derive(Debug)]
pub enum Error {
    DataAccess(data_access::Error),
	SerdeJson(String)
}

impl From<data_access::Error> for Error {
	fn from(err: data_access::Error) -> Self {
		Self::DataAccess(err)
	}
}

impl From<serde_json::Error> for Error {
	fn from(err: serde_json::Error) -> Self {
		Self::SerdeJson(err.to_string())
	}
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