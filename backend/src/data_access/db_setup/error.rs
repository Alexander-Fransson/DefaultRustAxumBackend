use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    Sqlx(
        #[serde_as(as = "DisplayFromStr")]
        sqlx::Error
    ),
	Migration(
		#[serde_as(as = "DisplayFromStr")]
		sqlx::migrate::MigrateError
	),
    FailedToReadFiles(String),   
}

impl From<sqlx::Error> for Error {
	fn from(err: sqlx::Error) -> Self {
		Self::Sqlx(err)
	}
}

impl From<sqlx::migrate::MigrateError> for Error {
	fn from(err: sqlx::migrate::MigrateError) -> Self {
		Self::Migration(err)
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