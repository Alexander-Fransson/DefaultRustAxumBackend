use std::fs;
use crate::utils::db_utils::create_connection_pool;
use crate::config_env::get_env_variables;

pub async fn reset_db() -> Result<(), sqlx::Error> {

    let connect_as_default_user = &get_env_variables().DB_DEFAULT_USER_CONNECTION_STRING;
    //let pool = sqlx::postgres::PgPool::connect(connection).await?;

    let pool = create_connection_pool(connect_as_default_user).await?;

    let test = sqlx::query("SELECT 1 + 1  AS sum")
    .fetch_one(&pool)
    .await?;   

    println!("connected to db {:?}", test);

    // I need to create a default user and then create a task user whcih can be dropped and recreated

    // what Ive learned is that you cannot delete a db and user when you have a connection to it
    // and you cannot use drop db in a transaction 

    let file_data  = fs::read_to_string("./db/sql/reset_db/00_recreate_db.sql")?;
    let recreation_commands: Vec<&str> = file_data.split(";").collect();

    for command in recreation_commands { 
        sqlx::query(command)
        .execute(&pool)
        .await?;
    }


    let connect_as_service_user = &get_env_variables().DB_CONNECTION_STRING;
    let pool = create_connection_pool(connect_as_service_user).await?;

    sqlx::migrate!("./db/sql/migrations")
    .run(&pool)
    .await?;

    let test2 = sqlx::query("SELECT * FROM \"user\"")
    .fetch_all(&pool)
    .await?;

    println!("users: {:#?}", test2);

    //Terminate connections to the database
   
    Ok(())
}