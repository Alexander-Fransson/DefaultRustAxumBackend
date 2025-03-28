use std::fmt::Debug;
use crate::data_access;
use crate::utils;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CannotFindEnvWithSuchName(&'static str),
    FailedToParse(&'static str),
	DataAccessError(data_access::Error),
	Io(std::io::Error),
	Utils(utils::Error)
}

impl From<utils::Error> for Error {
	fn from(err: utils::Error) -> Self {
		Self::Utils(err)
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