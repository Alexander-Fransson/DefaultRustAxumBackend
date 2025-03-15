pub mod user_controller_test;

use crate::views::user::{User, UserForRegister};
use crate::data_access::{
    base_crud::{self, Controller}, 
    DataAccessManager, 
    Result
};

pub struct UserController;

impl Controller for UserController {
    const TABLE_NAME: &'static str = "\"user\"";
}

impl UserController {
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
}