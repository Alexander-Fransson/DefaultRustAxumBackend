use super::error::{Error, Result};
use sqlx::{postgres::PgRow, FromRow};
use crate::utils::traits_for_proc_macros::GetStructFields;
use proc_macros::GetStructFields;

use super::DataAccessManager;

pub trait Controller {
    const TABLE_NAME: &'static str;
}

pub async fn get<C, T>(db: &DataAccessManager, id: i64) -> Result<T>
where 
C: Controller, 
T: for<'r> FromRow<'r, PgRow> + // ensures that the struct implements FromRow so that sqlx can map the result to the struct
Unpin + Send + GetStructFields // to make it work well with async functions
{

    let connection = db.get_db_connection();
    let struct_fields = T::get_struct_fields();
    let struct_string = struct_fields.join(", ");

    let query_string = format!("SELECT {} FROM {} WHERE id = $1", struct_string, C::TABLE_NAME);

    let row: T = sqlx::query_as(&query_string)
    .bind(id)
    .fetch_one(connection)
    .await
    .map_err(|e| Error::QueryFailed(e))?;

    Ok(row)
}