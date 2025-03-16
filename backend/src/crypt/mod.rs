mod error;
mod crypt_test;
pub mod password;
pub use error::{Result, Error};

use base64::{
    engine::general_purpose::STANDARD,
    Engine
};

pub struct EncryptContent {
    pub content: String,
    pub salt: String
}

pub fn string_to_base_64(str: &str) -> String {
    STANDARD.encode(str)
}