mod error;
mod crypt_test;
pub mod password;
pub mod jwt_token;
pub use error::{Result, Error};

use blake2::{
    Blake2bMac512,
    digest::Mac
};
use crate::utils::base64::u8_to_b64;

#[derive(Debug)]
pub struct EncryptContent {
    pub content: String,
    pub salt: String
}


// this function creates a unique signature based on the key and the encryptio content
// It cannot be decrypted but will generate the same unique signature each time
// this makes it useful for making varifiable signatures
pub fn encrypt_blake2b_mac_512(key: &[u8], enc_content: &EncryptContent) -> Result<String> {
    
    let EncryptContent{content, salt} = enc_content;

    let mut hasher = Blake2bMac512::new_from_slice(key)
    .map_err(|e| Error::FailedToCreateBlake2bHasher(e.to_string()))?;

    hasher.update(content.as_bytes());
    hasher.update(salt.as_bytes());

    let result = hasher.finalize().into_bytes();

    Ok(u8_to_b64(&result))
}