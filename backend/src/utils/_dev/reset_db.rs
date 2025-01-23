use crate::config_env::get_env_variables;

pub async fn reset_db() -> Result<(), sqlx::Error> {

    let connection = &get_env_variables().DB_CONNECTION_STRING;
    let pool = sqlx::postgres::PgPool::connect(connection).await?;

    let test = sqlx::query("SELECT 1 + 1  AS sum")
    .fetch_one(&pool)
    .await?;   

    println!("connected to db {:?}", test);

    // migrate the reset db
    sqlx::migrate!("./db/sql/reset_db")
    .run(&pool)
    .await
    .unwrap();

    let test2 = sqlx::query("SELECT * FROM \"user\"")
    .fetch_one(&pool)
    .await?;  

    println!("user table: {:#?}", test2);

    Ok(())
}