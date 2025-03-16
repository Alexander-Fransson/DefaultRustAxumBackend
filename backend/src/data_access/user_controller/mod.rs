pub mod user_controller_test;

use uuid::Uuid;
use crate::utils::traits_for_proc_macros::GetStructFields;
use crate::crypt::{self, password, EncryptContent};
use crate::views::user::{FullUserForTest, User, UserForLogin, UserForRegister, UserForValidation};
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

    pub async fn login_user(db: &DataAccessManager, credentials: UserForLogin) -> Result<i64> {
        
        // get all users with an email and check if at least one is valid
        let UserForLogin{email, password} = credentials;
        let fields = UserForValidation::get_struct_fields().join(", ");
        let query = format!("SELECT {} FROM {} WHERE email = $1", fields ,Self::TABLE_NAME);
        let connection = db.get_db_connection();

        let users_with_email: Vec<UserForValidation> = sqlx::query_as(&query)
        .bind(email)
        .fetch_all(connection)
        .await
        .map_err(|e| Error::QueryFailed(e))?;
        
        println!("USERS {:?}", users_with_email);

        // checks if the password provided encrypted with the password encryption salt is the same as the users password
        for user in users_with_email {
            let salt_string = user.password_encryption_salt.to_string();
            let enc_content = EncryptContent {
                content: password.clone(),
                salt: crypt::string_to_base_64(&salt_string)
            };

            match password::validate_password(user.password, &enc_content) {
                Ok(()) => return Ok(user.id),
                Err(crypt::Error::PasswordInvalid) => continue,
                Err(e) => return Err(Error::Crypt(e))
            }
        }

        Err(Error::EntityNotFound)
    }
}