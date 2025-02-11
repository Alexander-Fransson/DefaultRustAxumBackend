use axum::extract::{State, Path};
use axum::Router;
use axum::Json;
use crate::data_access::user_controller::UserController;
use crate::data_access:: DataAccessManager;
use crate::data_shapes::user::{User, UserForRegister};
use super::Result;
use axum::routing::{get, post};

pub fn user_routes(da: DataAccessManager) -> Router {
    Router::new()
    .route("/user/", post(create_user_handler))
    .route("/user/{id}", get(get_user_handler).delete(delete_user_handler)) // in axum 0.7 you should use {id} instead of :id
    .with_state(da)
}

pub async fn get_user_handler(
    State(da): State<DataAccessManager>, 
    Path(id): Path<i64>
) -> Result<Json<User>> {

    let user = UserController::get(&da, id).await?;

    Ok(Json(user))
}

pub async fn create_user_handler(
    State(da): State<DataAccessManager>, 
    Json(user): Json<UserForRegister>
) -> Result<Json<User>> {

    let user_password = user.password.clone();
    let user_email = user.email.clone();
    let user_username = user.username.clone();

    let user_id = UserController::create(&da, user).await?;

    let user = User {
        id: user_id,
        username: user_username,
        email: user_email,
        password: user_password,
    };

    Ok(Json(user))
}

pub async fn delete_user_handler(
    State(da): State<DataAccessManager>, 
    Path(id): Path<i64>
) -> Result<Json<&'static str>> {

    UserController::delete(&da, id).await?;

    Ok(Json("user deleted"))
}