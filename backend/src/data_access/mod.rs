use sqlx::{Pool,Postgres};
use crate::db_setup::create_serive_user_connection_pool;

#[derive(Clone)]
pub struct DataAccessManager {
    db_connection: Pool<Postgres>,
}

// maybe the data access manager should house the db setup stuff.
// although the reet db probably does not belong

// impl DataAccessManager {
//     pub fn new() -> Result<Self> {
//         let data_access_manager = DataAccessManager {
//             db_connection: create_serive_user_connection_pool().await?,
//         };

//         Ok(data_access_manager)
//     }
// }
