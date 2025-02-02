use super::error::{Error, Result};
use sqlx::{postgres::PgRow, FromRow};

use super::DataAccessManager;


pub trait Controller {
    const TABLE_NAME: &'static str;
}


pub async fn get<C, T>(db: &DataAccessManager, id: i64) -> Result<T>
where 
C: Controller, 
T: for<'r> FromRow<'r, PgRow> + // ensures that the struct implements FromRow so that sqlx can map the result to the struct
Unpin + Send // to make it work well with async functions
{

    let connection = db.get_db_connection();



    todo!()
}