use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use proc_macros::GetStructFields;
use crate::utils::traits_for_proc_macros::GetStructFields;

// maybe this shall be moved to a dedicated model_structs folder to allow the frontend to use them
// although for axum only this django like structure might suffice

// will this ever be used?
#[derive(Serialize, Deserialize, FromRow, GetStructFields)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, FromRow, Clone, Debug, GetStructFields)]
pub struct Username {
    pub username: String
}

#[derive(Serialize, Deserialize)]
pub struct UserForRegister {
    pub username: String,
    pub email: String,
    pub password: String
}

// might be kind to let the user use either the email or the username
// #[derive(Deserialize)]
// pub struct UserForLogin {
//     pub email: String,
//     pub password: String
// }