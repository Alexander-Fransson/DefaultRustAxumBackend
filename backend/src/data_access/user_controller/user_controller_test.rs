#[cfg(test)]
mod tests {
    use serial_test::serial;

    // sometimes the pool times out, the tests work individually though

    use crate::data_access::_get_data_access_manager_for_tests;
    use crate::views::user::{User, UserForLogin, UserForRegister};
    use crate::data_access::{Result, Error};
    use super::super::UserController;

    const TEST_EMAIL: &str = "aadawfawfaknwalkjgnwangagjnkawnjgkajfnkajds,mnfaf_unique_email@example.com";
    const TEST_PASSWORD: &str = "test_password";

    fn generate_user_for_register() -> UserForRegister {
        UserForRegister {
            name: "test_user2".to_string(),
            email: TEST_EMAIL.to_string(),
            password: TEST_PASSWORD.to_string(),
        }
    }

    #[serial]
    #[tokio::test]
    async fn user_create_get_delete_ok() -> Result<()> {
        let db = _get_data_access_manager_for_tests().await;

        let new_user = generate_user_for_register();
        
        let create_req_id = UserController::create(&db, new_user).await?;

        let user= UserController::get::<User>(&db, create_req_id).await?;

        assert_eq!(user.name, "test_user2");

        UserController::delete(&db, user.id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn list_by_name_ok() -> Result<()> {
        let db = _get_data_access_manager_for_tests().await;

        let users_vec = UserController::list_by_name(&db, "test").await?;

        println!("\nUSERS: {:?}\n", users_vec);

        assert!(users_vec.len() != 0);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn register_user_ok() -> Result<()> {
        let db = _get_data_access_manager_for_tests().await;

        let new_user = generate_user_for_register();
        
        let create_req = UserController::register_user(&db, new_user).await?;

        let user = UserController::_display_full_user(&db, create_req.id).await?;

        assert_eq!(user.name, "test_user2");
        assert_eq!(user.email, TEST_EMAIL);
        assert!(user.password.starts_with("#0#$argon2id")); // is encrypted with argon2
        assert_eq!(user.password_encryption_salt.get_version_num(), 4); // uuid of type 4

        UserController::delete(&db, user.id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn login_user_ok() -> Result<()> {
        let db = _get_data_access_manager_for_tests().await;

        let new_user = generate_user_for_register();
        let create_req = UserController::register_user(&db, new_user).await?;

        let login_credentials = UserForLogin {
            email: TEST_EMAIL.to_string(),
            password: TEST_PASSWORD.to_string(),
        };

        let login_res = UserController::login_user(&db, login_credentials.clone()).await?;

        assert_eq!(login_res.id, create_req.id);

        let bad_password_credentials = UserForLogin {
            email: TEST_EMAIL.to_string(),
            password: "bad_password".to_string(),
        };

        let bad_login_res = UserController::login_user(&db, bad_password_credentials).await;
        assert!(matches!(bad_login_res, Err(Error::IncorrectPassword)));

        UserController::delete(&db, create_req.id).await?;

        let not_found_login_res = UserController::login_user(&db, login_credentials).await;
        assert!(matches!(not_found_login_res, Err(Error::EntityNotFound)));

        Ok(())
    }

    // maybe test that incorrect passwords are rejected with apropriate error
}