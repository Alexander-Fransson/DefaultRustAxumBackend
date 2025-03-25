use super::{Error, Result, EncryptContent};
use argon2::{
    Argon2,
    password_hash::{
        PasswordHasher,
        SaltString
    }
};

pub fn hash_password(content: &EncryptContent) -> Result<String> {
    
    let EncryptContent {content, salt} = content;

    // postgres might add padding "=" to ensure the length is a multiple of 4
    // argon2 does not accept padding however
    let trimmed_salt = salt.trim_end_matches("=");  

    let argon2 = Argon2::default();
    let salt_string = SaltString::from_b64(trimmed_salt)
    .map_err(|e| Error::FailedToTurnPasswordSaltIntoSaltString(e))?;

    let hashed_password = argon2.hash_password(
        content.as_bytes(), 
        &salt_string
    ).map_err(|e| Error::FailedToHashPassword(e))?;

    // we add a #1# if there will come a better alternative than argon2
    // then we can match the different encryption methods with the #n#
    Ok(format!("#0#{}", hashed_password.to_string()))
}

pub fn validate_password(password_ref:String, enc_content: &EncryptContent) -> Result<()> {
    let password = hash_password(enc_content)?;
    if password == password_ref {Ok(())} 
    else {Err(Error::PasswordInvalid)}
}