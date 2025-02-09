use axum::extract::{State, Path};
use axum::Router;
use axum::Json;
use crate::data_access::user_controller::UserController;
use crate::data_access::DataAccessManager;
use crate::data_shapes::user::User;
use super::{Result, Error};

pub async fn user_routes() -> Router {
    Router::new()
}

pub async fn get_user_handler(
    State(da): State<DataAccessManager>, 
    Path(id): Path<i64>
) -> Result<Json<User>> {

    let user = UserController::get(&da, id).await?;


    Ok(Json(user))
}