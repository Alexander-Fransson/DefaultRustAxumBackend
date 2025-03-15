use super::error::{Error, Result};
use serde::Serialize;
use sqlx::{postgres::PgRow, FromRow};
use crate::utils::traits_for_proc_macros::GetStructFields;
use crate::utils::turn_struct_with_serde_serialize_into_hashmap;

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

    let struct_string = T::get_struct_fields().join(", ");
    let query_string = format!("SELECT {} FROM {} WHERE id = $1", struct_string, C::TABLE_NAME);

    let row: T = sqlx::query_as(&query_string)
    .bind(id)
    .fetch_one(connection)
    .await
    .map_err(|e| Error::QueryFailed(e))?;

    Ok(row)
}

pub async fn list<C, T>(db: &DataAccessManager) -> Result<Vec<T>>
where 
C: Controller, 
T: for<'r> FromRow<'r, PgRow> +
Unpin + Send + GetStructFields 
{
    let connection = db.get_db_connection();

    let struct_string = T::get_struct_fields().join(", ");
    let query_string = format!("SELECT {} FROM {}", struct_string, C::TABLE_NAME);

    let rows: Vec<T> = sqlx::query_as(&query_string)
    .fetch_all(connection)
    .await
    .map_err(|e| Error::QueryFailed(e))?;

    Ok(rows)
}

// This is not useful fore the USER but it is useful for other entities

pub async fn list_by_name<C, T>(db: &DataAccessManager, name: &str) -> Result<Vec<T>>
where 
C: Controller, 
T: for<'r> FromRow<'r, PgRow> +
Unpin + Send + GetStructFields 
{
    let connection = db.get_db_connection();

    let struct_string = T::get_struct_fields().join(", ");
    let query_string = format!("SELECT {} FROM {} WHERE name ILIKE $1", struct_string, C::TABLE_NAME);

    let rows: Vec<T> = sqlx::query_as(&query_string)
    .bind(format!("%{}%", &name))
    .fetch_all(connection)
    .await
    .map_err(|e| Error::QueryFailed(e))?;

    Ok(rows)
}

pub async fn create<C, T>(db: &DataAccessManager, data: T) -> Result<i64>
where 
C: Controller, 
T: Serialize {
    let connection = db.get_db_connection();

    let data_hash_map = turn_struct_with_serde_serialize_into_hashmap(data)?;
    let keys = data_hash_map.keys().into_iter().map(|k| k.to_string()).collect::<Vec<_>>();
    let values = data_hash_map.values().into_iter().map(|v| format!("'{}'", v)).collect::<Vec<_>>();

    let query_string = format!("INSERT INTO {} ({}) VALUES ({}) RETURNING id", C::TABLE_NAME, keys.join(", "), values.join(", "));

    let (id,) = sqlx::query_as::<_,(i64,)>(&query_string)
    .fetch_one(connection)
    .await
    .map_err(|e| Error::QueryFailed(e))?;

    Ok(id)
}

pub async fn delete<C, T>(db: &DataAccessManager, id: i64) -> Result<()> where C: Controller {
    let connection = db.get_db_connection();

    let rows_affected = sqlx::query(&format!("DELETE FROM {} WHERE id = $1", C::TABLE_NAME))
    .bind(id)
    .execute(connection)
    .await?
    .rows_affected();

    if rows_affected == 0 {
        return Err(Error::EntityNotFound);
    }

    Ok(())
}