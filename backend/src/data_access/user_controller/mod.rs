pub mod user_controller_test;

use uuid::Uuid;
use crate::crypt::{self, EncryptContent};
use crate::views::user::{FullUserForTest, User, UserForRegister};
use crate::data_access::{
    base_crud::{self, Controller}, 
    DataAccessManager, 
    Result,
    Error
};

pub struct UserController;

impl Controller for UserController {
    const TABLE_NAME: &'static str = "\"user\"";
}

impl UserController {

    // most of these are only for demonstration purpuses

    pub async fn get(db: &DataAccessManager, id: i64) -> Result<User> {
        let user = base_crud::get::<Self, _>(db, id).await?;
        Ok(user)
    }

    pub async fn create(db: &DataAccessManager, user: UserForRegister) -> Result<i64> {
        let user_id = base_crud::create::<Self, UserForRegister>(db, user).await?;
        Ok(user_id)
    }

    pub async fn delete(db: &DataAccessManager, id: i64) -> Result<()> {
        base_crud::delete::<Self, User>(db, id).await?;
        Ok(())
    }

    pub async fn list_by_name(db: &DataAccessManager, name: &str) -> Result<Vec<User>> {
        let users = base_crud::list_by_name::<Self, _>(db, name).await?;
        Ok(users)
    }

    pub async fn _display_full_user(db: &DataAccessManager, id: i64) -> Result<FullUserForTest> {
        let user = base_crud::get::<Self, _>(db, id).await?;
        Ok(user)
    }

    pub async fn register_user(db: &DataAccessManager, user: UserForRegister) -> Result<i64> {

        let pwd_salt_uuid = Uuid::new_v4();
        let pwd_salt_b64 = crypt::string_to_base_64(&pwd_salt_uuid.to_string());

        let enc_content = EncryptContent {
            content: user.password,
            salt: pwd_salt_b64
        };
        let encrypted_password = crypt::password::hash_password(&enc_content)?;

        let query = format!("INSERT INTO {} (name, email, password, password_encryption_salt) VALUES ($1, $2, $3, $4) RETURNING id", Self::TABLE_NAME);
        let connection = db.get_db_connection();

        let (user_id,) = sqlx::query_as::<_, (i64,)>(&query)
        .bind(user.name)
        .bind(user.email)
        .bind(encrypted_password)
        .bind(pwd_salt_uuid)
        .fetch_one(connection)
        .await
        .map_err(|e| Error::QueryFailed(e))?;

        Ok(user_id)
    }
}