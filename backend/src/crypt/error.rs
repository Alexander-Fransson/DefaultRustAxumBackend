use std::fmt::Debug;
use argon2::password_hash;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FailedToTurnPasswordSaltIntoSaltString(password_hash::Error),
    FailedToHashPassword(password_hash::Error),
	FailedToDecodeBase64(base64::DecodeError),
	PasswordInvalid
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