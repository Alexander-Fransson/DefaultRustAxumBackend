#[cfg(test)]
pub mod tests {

    use crate::crypt::password::validate_password;

    use super::super::{
        Result,
        Error,
        password::hash_password,
        EncryptContent,
        encrypt_blake2b_mac_512
    };
     
    #[test]
    fn encrypt_blake2b_mac_512_ok() -> Result<()> {
        
        let test_key = [0u8; 64];

        let test_enc_content = EncryptContent {
            content: "somecontent".to_string(),
            salt: "electrolytes".to_string()
        };

        let signature = encrypt_blake2b_mac_512(&test_key, &test_enc_content)?;

        assert_eq!(&signature, "#1#3InVh31+qBSdgCzxOD6bUigENgiJcza+BfA6Uj2ETWZ6geu0ID5vOIG/CGiB2gg5eBMeh9Map4GeuBasuQbpsQ==");
        
        Ok(())
    }

    #[test]
    fn hash_password_ok() -> Result<()> {

        let encryption_content = EncryptContent {
            content: "somecontent".to_string(),
            salt: "aGVsbG8gd29ybGR+Cg==".to_string()
        };
        

        let hashed_password = hash_password(&encryption_content)?;
        
        assert_eq!(&hashed_password,"#0#$argon2id$v=19$m=19456,t=2,p=1$aGVsbG8gd29ybGR+Cg$KNQ4cSSFwLmzqDgtJ6SnIJe6ElCTk3peC2ui4LyI0OA");
        
        Ok(())
    }

    #[test]
    fn validate_password_ok() -> Result<()> {
       
        let password_ref = "#0#$argon2id$v=19$m=19456,t=2,p=1$aGVsbG8gd29ybGR+Cg$KNQ4cSSFwLmzqDgtJ6SnIJe6ElCTk3peC2ui4LyI0OA".to_string();

        let enc_content = EncryptContent {
            content: "somecontent".to_string(),
            salt: "aGVsbG8gd29ybGR+Cg==".to_string()
        };
       
        validate_password(password_ref, &enc_content)?;

        let failed_password = validate_password("wrongpassword".to_string(), &enc_content);

        if let Err(Error::PasswordInvalid) = failed_password {
            assert!(true);
        } else {
            assert!(false);
        }

        Ok(()) 
    }   
}