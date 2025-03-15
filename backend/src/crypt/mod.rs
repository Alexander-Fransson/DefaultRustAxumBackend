mod error;
mod crypt_test;
pub use error::{Result, Error};
use argon2::{
    Argon2,
    password_hash::{
        PasswordHash,
        PasswordHasher,
        SaltString
    }
};

pub struct EncryptContent {
    pub content: String,
    pub salt: String
}

pub fn hash_password(content: &EncryptContent) -> Result<String> {
    
    let EncryptContent {content, salt} = content;

    let argon2 = Argon2::default();
    let salt_string = SaltString::from_b64(salt.as_str())
    .map_err(|_| Error::FailedToTurnPasswordSaltIntoSaltString)?;

    let hashed_password = argon2.hash_password(
        content.as_bytes(), 
        &salt_string
    ).map_err(|e| Error::FailedToHashPassword(e.to_string()))?;

    Ok(hashed_password.to_string())
}