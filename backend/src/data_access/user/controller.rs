use crate::data_access::{
    base::{self, Controller}, DataAccessManager, Error, Result
};

use super::model_structs::{User, UserForRegister};

pub struct UserController;

impl Controller for UserController {
    const TABLE_NAME: &'static str = "\"user\"";
}

impl UserController {
    pub async fn get(db: &DataAccessManager, id: i64) -> Result<User> {
        let user = base::get::<UserController, User>(db, id).await?;
        Ok(user)
    }

    pub async fn create(db: &DataAccessManager, user: UserForRegister) -> Result<i64> {
        let user_id = base::create::<UserController, UserForRegister>(db, user).await?;
        Ok(user_id)
    }

    pub async fn delete(db: &DataAccessManager, id: i64) -> Result<()> {
        base::delete::<UserController, User>(db, id).await
    }
}