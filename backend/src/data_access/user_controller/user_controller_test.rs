#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::data_access::_get_data_access_manager_for_tests;
    use crate::views::user::{UserForLogin, UserForRegister};
    use crate::data_access::Result;
    use super::super::UserController;

    fn get_user_for_register() -> UserForRegister {
        UserForRegister {
            name: "test_user2".to_string(),
            email: "email@example.com3333".to_string(),
            password: "test_password24".to_string(),
        }
    }

    #[serial]
    #[tokio::test]
    async fn user_create_get_delete_ok() -> Result<()> {
        let db = _get_data_access_manager_for_tests().await;

        let new_user = get_user_for_register();
        
        let create_req_id = UserController::create(&db, new_user).await?;

        let user = UserController::get(&db, create_req_id).await?;

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

        let new_user = get_user_for_register();
        
        let create_req_id = UserController::register_user(&db, new_user).await?;

        let user = UserController::_display_full_user(&db, create_req_id).await?;

        assert_eq!(user.name, "test_user2");
        assert_eq!(user.email, "email@example.com3333");
        assert!(user.password.starts_with("#0#$argon2id")); // is encrypted with argon2
        assert_eq!(user.password_encryption_salt.get_version_num(), 4); // uuid of type 4

        UserController::delete(&db, user.id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn login_user_ok() -> Result<()> {
        let db = _get_data_access_manager_for_tests().await;

        let new_user = get_user_for_register();
        let create_req_id = UserController::register_user(&db, new_user).await?;

        let login_credentials = UserForLogin {
            email: "email@example.com3333".to_string(),
            password: "test_password24".to_string(),
        };

        let login_res_id = UserController::login_user(&db, login_credentials).await?;

        assert_eq!(login_res_id, create_req_id);

        UserController::delete(&db, create_req_id).await?;

        Ok(())
    }
}