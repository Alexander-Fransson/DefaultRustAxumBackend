#[cfg(test)]
mod tests {
    use crate::data_access::_get_data_access_manager_for_tests;
    use crate::data_shapes::user::UserForRegister;
    use crate::data_access::Result;
    use super::super::UserController;

    #[tokio::test]
    #[ignore]
    async fn user_create_get_delete_ok() -> Result<()> {
        let db = _get_data_access_manager_for_tests().await;

        let new_user = UserForRegister {
            username: "test_user2".to_string(),
            email: "email@example.com3333".to_string(),
            password: "test_password24".to_string(),
        };

        // sqlx::query("INSERT INTO \"user\" (email, username,  \"password\") VALUES ('email@example.com333333', 'test_user', 'test_password')")
        // .execute(db.get_db_connection())
        // .await?;
        
        let create_req_id = UserController::create(&db, new_user).await?;

        println!("create_req_id: {}", create_req_id);

        let user = UserController::get(&db, create_req_id).await?;

        assert_eq!(user.username, "test_user2");

        UserController::delete(&db, user.id).await?;

        Ok(())
    }
}