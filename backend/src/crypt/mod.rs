mod error;
mod crypt_test;
pub mod password;
pub use error::{Result, Error};

pub struct EncryptContent {
    pub content: String,
    pub salt: String
}