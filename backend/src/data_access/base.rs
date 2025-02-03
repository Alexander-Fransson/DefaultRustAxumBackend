use super::error::{Error, Result};
use serde::Serialize;
use sqlx::{postgres::PgRow, FromRow};

use super::DataAccessManager;


pub trait Controller {
    const TABLE_NAME: &'static str;
}


pub async fn get<C, T>(db: &DataAccessManager, id: i64) -> Result<T>
where 
C: Controller, 
T: for<'r> FromRow<'r, PgRow> + // ensures that the struct implements FromRow so that sqlx can map the result to the struct
Unpin + Send + Serialize // to make it work well with async functions
{

    let connection = db.get_db_connection();

    let query_string = format!("SELECT * FROM {} WHERE id = $1", C::TABLE_NAME);

    todo!()
}