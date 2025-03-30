use crate::utils::time::now_utc_plus_sec_str;

use super::super::Result;
use super::{
    validate_jwt_token_by_signature_and_expiration,
    create_jwt_token,
    create_token_signature,
};

#[test]
fn test_create_sign_and_validate_jwt_token() -> Result<()> {
    let user_id = 1;
    let salt = "salt";
    let jwt_key = [0u8; 64];
    let durration_sec = 2.0;

    let token = create_jwt_token(user_id, salt, &jwt_key, durration_sec)?;

    let reference_signature = create_token_signature(user_id, &token.expiration, salt, &jwt_key)?;

    assert_eq!(&token.signature, &reference_signature);
    assert_eq!(token.user_id, user_id);
    assert!(token.expiration <= now_utc_plus_sec_str(durration_sec)?);
    assert_eq!(token.expiration, token.expiration);

    validate_jwt_token_by_signature_and_expiration(&token, salt, &jwt_key)?;

    Ok(())
}