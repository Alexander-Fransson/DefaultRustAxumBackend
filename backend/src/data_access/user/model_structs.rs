use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
// maybe this shall be moved to a dedicated model_structs folder to allow the frontend to use them
// although for axum only this django like structure might suffice

// will this ever be used?
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, FromRow, Clone, Debug)]
pub struct Username {
    pub username: String
}

#[derive(Deserialize)]
pub struct UserForRegister {
    pub username: String,
    pub email: String,
    pub password: String
}

// might be kind to let the user use either the email or the username
#[derive(Deserialize)]
pub struct UserForLogin {
    pub email: String,
    pub password: String
}