#![allow(unused_variables)]

use std::fmt::Display;
use std::str::FromStr;
use super::{EncryptContent, Error, Result};
use crate::utils::base64::{b64_to_string, string_to_base_64};

// ignore never read
#[allow(dead_code)]

pub struct JwtToken {
    pub user_id: i64,
    pub expiration: String,
    pub b64_signature: String
}

impl FromStr for JwtToken {
    type Err = Error;

    fn from_str(token_str: &str) -> Result<Self> {

        let token_parts = token_str.split(".")
        .collect::<Vec<&str>>();

        if token_parts.len() != 3 {
            return Err(Error::JwtTokenWrongFormat);
        }

        let (
            b64_user_id,
            b64_expiration,
            b64_signature
        ) = (token_parts[0], token_parts[1], token_parts[2]);

        let user_id = b64_to_string(b64_user_id)?.parse::<i64>()
        .map_err(|e| Error::FailedToParseUserIdToB64(e.to_string()))?;

        let expiration = b64_to_string(b64_expiration)?;

        Ok(Self {
            user_id,
            expiration,
            b64_signature: b64_signature.to_string()
        })
    }
}

impl Display for JwtToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}",
            string_to_base_64(&self.user_id.to_string()),
            string_to_base_64(&self.expiration), 
            self.b64_signature
        )
    }
}

fn _create_token_signature(
    user_id: i64, 
    expiration: &str, 
    token_salt: &str, 
    encryption_key: &[u8]
) -> String {
    // create an encryption with blake 2b function
    todo!()
}

fn encrypt_token(enc_content: &EncryptContent) -> String {
    todo!()
}