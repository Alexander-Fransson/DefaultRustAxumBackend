mod db_setup;
pub mod error;
pub mod base_crud;
pub mod user_controller;

pub use error::{Error, Result};

use sqlx::{Pool,Postgres};
use db_setup::{create_serive_user_connection_pool, make_migrations, _reset_db};
use tokio::sync::OnceCell;

#[derive(Clone)]
pub struct DataAccessManager {
    db_connection: Pool<Postgres>,
}

impl DataAccessManager {
    pub async fn new() -> Result<Self> {
        let connection = create_serive_user_connection_pool().await?;

        _reset_db().await?; // comment out if you dont want db reset on startup

        make_migrations(&connection).await?;
        
        let data_access_manager = DataAccessManager {
            db_connection: connection,
        };

        Ok(data_access_manager)
    }

    // a function that can only be used in the data_access module
    pub (in crate::data_access) fn get_db_connection(&self) -> &Pool<Postgres> {
        &self.db_connection
    }
}

pub async fn _get_data_access_manager_for_tests() -> DataAccessManager {
    static DB: OnceCell<DataAccessManager> = OnceCell::const_new();

    let db = DB.get_or_init(|| async {
        DataAccessManager::new().await.unwrap()
    }).await;

    db.clone()
}
