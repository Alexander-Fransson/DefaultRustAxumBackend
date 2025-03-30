#[cfg(test)]
mod tests;

use std::fmt::Display;
use std::str::FromStr;
use time::OffsetDateTime;
use crate::utils::time::time_str_to_offset_date_time;
use super::{
    EncryptContent, 
    Error, 
    Result, 
    encrypt_blake2b_mac_512
};
use crate::{
    config_env::get_env_variables, 
    utils::{
        base64::{
            b64_to_string, 
            string_to_base_64
        }, 
        time::now_utc_plus_sec_str
    }
};

pub struct JwtToken {
    pub user_id: i64,
    pub expiration: String,
    pub signature: String
}

impl JwtToken {
    pub fn new(user_id: i64, salt: &str) -> Result<Self> {
        let jwt_key = &get_env_variables().JWT_KEY;
        let durration_sec = get_env_variables().JWT_TOKEN_DURRATION_SEC;

        create_jwt_token(user_id, salt, jwt_key, durration_sec)
    }

    // maybe having the validate by expiration in another function could be better for testing
    pub fn validate(&self, token_salt: &str) -> Result<()> {
        let jwt_key = &get_env_variables().JWT_KEY;

        validate_jwt_token_by_signature_and_expiration(self, token_salt, jwt_key)
    }
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
            signature: b64_signature.to_string()
        })
    }
}

impl Display for JwtToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}",
            string_to_base_64(&self.user_id.to_string()),
            string_to_base_64(&self.expiration), 
            self.signature
        )
    }
}

fn create_token_signature(
    user_id: i64, 
    expiration: &str, 
    token_salt: &str, 
    jwt_key: &[u8]
) -> Result<String> {

    // does this realy have to be b64
    let b64_id = string_to_base_64(&user_id.to_string());
    let b64_expiration = string_to_base_64(expiration);

    // should I use user_id and salt as the content, I dont compare using them both so one should be enough
    let content = format!("{}.{}", b64_id, b64_expiration);
    let enc_content = EncryptContent {
        content,
        salt: token_salt.to_string()
    };

    encrypt_blake2b_mac_512(
        jwt_key, 
        &enc_content
    )
}

// testable standard functions becouse having an answer and a source for encryption is sort of a security flaw

fn create_jwt_token(user_id: i64, salt: &str, jwt_key: &[u8], durration_sec: f64) -> Result<JwtToken> {
    let expiration = now_utc_plus_sec_str(durration_sec)?;
    let signature = create_token_signature(user_id, &expiration, salt, jwt_key)?;
    
    Ok(JwtToken{
        user_id,
        expiration,
        signature
    })  
}

fn validate_jwt_token_by_signature_and_expiration(token: &JwtToken, token_salt: &str, jwt_key: &[u8]) -> Result<()> {
    
    let reference_signature = create_token_signature(token.user_id, &token.expiration, token_salt, jwt_key)?;

    if reference_signature != token.signature {
        return Err(Error::JwtTokenInvalidSignature);
    }
    
    let expiration = time_str_to_offset_date_time(&token.expiration)
    .map_err(|e| Error::JwtTokenExpirationWrongFormat(e.to_string()))?;

    if expiration < OffsetDateTime::now_utc() {
        return Err(Error::JwtTokenExpired);
    } 

    Ok(())
}
