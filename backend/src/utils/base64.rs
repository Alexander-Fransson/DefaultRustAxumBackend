
use base64::{
    engine::general_purpose::STANDARD,
    Engine
};
use super::{Result, Error};
use std::str;

pub fn string_to_base_64(str: &str) -> String {
    STANDARD.encode(str)
}

pub fn b64_to_string(b64: &str) -> Result<String> {
    let bytes = STANDARD.decode(b64)
    .map_err(|e| Error::FailedToDecodeB64(e.to_string()))?;

    let string_from_b64 = str::from_utf8(&bytes)
    .map_err(|e| Error::FailedToDecodeB64Bytes(e.to_string()))?
    .to_string();

    Ok(string_from_b64)
}

pub fn b64_to_u8(b64: &str) -> Result<Vec<u8>> {
    let bytes = STANDARD.decode(b64)
    .map_err(|e| Error::FailedToDecodeB64(e.to_string()))?;
    
    Ok(bytes)
}

pub fn u8_to_b64(bytes: &[u8]) -> String {
    STANDARD.encode(bytes)
}