use std::fmt::Debug;
use super::db_setup;
use crate::utils;
use crate::crypt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
	DbSettupError(String),
	QueryFailed(String),
	Utils(String),
	Crypt(String),
	EntityNotFound,

	// login
	IncorrectPassword,

	// jwt
	JwtTokenWrongFormat,
}

impl From<crypt::Error> for Error {
	fn from(err: crypt::Error) -> Self {
		Self::Crypt(err.to_string())
	}
}

impl From<db_setup::Error> for Error {
	fn from(err: db_setup::Error) -> Self {
		Self::DbSettupError(err.to_string())
	}
}

impl From<sqlx::Error> for Error {
	fn from(err: sqlx::Error) -> Self {
		Self::QueryFailed(err.to_string())
	}
}

impl From<utils::error::Error> for Error {
	fn from(err: utils::error::Error) -> Self {
		Self::Utils(err.to_string())
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