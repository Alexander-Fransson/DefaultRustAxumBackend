use std::fmt::Debug;
use argon2::password_hash;

use crate::utils;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	// password
    FailedToTurnPasswordSaltIntoSaltString(password_hash::Error),
    FailedToHashPassword(password_hash::Error),
	FailedToDecodeBase64(base64::DecodeError),
	PasswordInvalid,

	// jwt
	JwtTokenWrongFormat,
	JwtB64DecodingError(utils::Error),
	FailedToParseUserIdToB64(String)

}

impl From<utils::Error> for Error {
	fn from(value: utils::Error) -> Self {
		Error::JwtB64DecodingError(value)
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