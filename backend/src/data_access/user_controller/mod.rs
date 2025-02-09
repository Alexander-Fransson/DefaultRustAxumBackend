pub mod user_controller_test;

use crate::data_shapes::user::{User, UserForRegister};
use crate::data_access::{
    base_crud::{self, Controller}, 
    DataAccessManager, 
    Error, 
    Result
};

pub struct UserController;

impl Controller for UserController {
    const TABLE_NAME: &'static str = "\"user\"";
}

impl UserController {
    pub async fn get(db: &DataAccessManager, id: i64) -> Result<User> {
        let user = base_crud::get::<UserController, User>(db, id).await?;
        Ok(user)
    }

    pub async fn create(db: &DataAccessManager, user: UserForRegister) -> Result<i64> {
        let user_id = base_crud::create::<UserController, UserForRegister>(db, user).await?;
        Ok(user_id)
    }

    pub async fn delete(db: &DataAccessManager, id: i64) -> Result<()> {
        base_crud::delete::<UserController, User>(db, id).await
    }
}