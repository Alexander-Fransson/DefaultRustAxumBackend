use serde_json;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    
	// struct to hashmap
	SerdeJson(String),
	ExpectedStruct,
	FailedToTurnJsonValueIntoMap,
	FailedToTurnJsonValueIntoStr,
	
	// time
	FailedToFormtOffsetDateTime(String),
	FailedToParseOffsetDateTime(String),

	// b64
	FailedToDecodeB64(String),
	FailedToDecodeB64Bytes(String)
}

impl From<serde_json::error::Error> for Error {
	fn from(err: serde_json::error::Error) -> Self {
		Self::SerdeJson(err.to_string())
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