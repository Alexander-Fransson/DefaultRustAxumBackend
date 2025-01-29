use std::fmt::Debug;
use crate::db_setup;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CannotFindEnvWithSuchName(&'static str),
    FailedToParse(&'static str),
	DbSettupError(db_setup::Error),
	Io(std::io::Error)
}

impl From<db_setup::Error> for Error {
	fn from(err: db_setup::Error) -> Self {
		Self::DbSettupError(err)
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