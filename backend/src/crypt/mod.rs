mod error;
mod crypt_test;
pub mod password;
pub mod jwt_token;
pub use error::{Result, Error};

#[derive(Debug)]
pub struct EncryptContent {
    pub content: String,
    pub salt: String
}

