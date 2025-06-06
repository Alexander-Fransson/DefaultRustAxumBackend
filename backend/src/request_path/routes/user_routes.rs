use axum::extract::{State, Path};
use axum::Router;
use axum::Json;
use tracing::info;
use crate::data_access::user_controller::UserController;
use crate::data_access:: DataAccessManager;
use crate::request_path::{Result, Error};
use crate::views::user::{User, UserForRegister};
use axum::routing::{get, post};

pub fn user_routes(da: DataAccessManager) -> Router {
    Router::new()
    .route("/user/", post(create_user_handler))
    .route("/user/{id}", get(get_user_handler).delete(delete_user_handler)) // in axum 0.7 you should use {id} instead of :id
    .with_state(da)
}

async fn get_user_handler(
    State(da): State<DataAccessManager>, 
    Path(id): Path<i64>
) -> Result<Json<User>> {

    let user = UserController::get(&da, id).await
    .map_err(|e| Error::DataAccess(e))?;

    Ok(Json(user))
}

async fn create_user_handler(
    State(da): State<DataAccessManager>, 
    Json(user): Json<UserForRegister>
) -> Result<Json<User>> {

    let user_password = user.password.clone();
    let user_email = user.email.clone();
    let user_username = user.name.clone();

    info!("user: {:#?}", user);

    let user_id = UserController::create(&da, user).await
    .map_err(|e| Error::DataAccess(e))?;

    let user = User {
        id: user_id,
        name: user_username,
        email: user_email,
        password: user_password,
    };

    Ok(Json(user))
}

async fn delete_user_handler(
    State(da): State<DataAccessManager>, 
    Path(id): Path<i64>
) -> Result<Json<&'static str>> {

    UserController::delete(&da, id).await
    .map_err(|e| Error::DataAccess(e))?;

    Ok(Json("user deleted"))
}