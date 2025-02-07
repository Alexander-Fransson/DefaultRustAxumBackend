#[cfg(test)]
mod tests {
    use crate::data_access::_get_data_access_manager_for_tests;
    use crate::data_access::user::model_structs::UserForRegister;
    use crate::data_access::Result;
    use super::super::controller::UserController;

    #[tokio::test]
    async fn user_create_get_delete_ok() -> Result<()> {
        let db = _get_data_access_manager_for_tests().await;

        let new_user = UserForRegister {
            username: "test_user".to_string(),
            email: "email@example.com".to_string(),
            password: "test_password".to_string(),
        };
        
        let create_req_id = UserController::create(&db, new_user).await?;

        let user = UserController::get(&db, create_req_id).await?;

        assert_eq!(user.username, "test_user");

        UserController::delete(&db, user.id).await?;

        Ok(())
    }
}