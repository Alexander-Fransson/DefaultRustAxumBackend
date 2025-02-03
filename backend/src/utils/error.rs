use serde_json;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SerdeJson(serde_json::error::Error),
	ExpectedStruct,
	FailedToTurnJsonValueIntoMap,
	FailedToTurnJsonValueIntoStr
}

impl From<serde_json::error::Error> for Error {
	fn from(err: serde_json::error::Error) -> Self {
		Self::SerdeJson(err)
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