use crate::config_env::get_env_variables;

pub async fn reset_db() -> Result<(), sqlx::Error> {

    let connection = &get_env_variables().DB_CONNECTION_STRING;
    let pool = sqlx::postgres::PgPool::connect(connection).await?;

    let test = sqlx::query("SELECT 1 + 1  AS sum")
    .fetch_one(&pool)
    .await?;   

    println!("connected to db {:?}", test);
    Ok(())
}