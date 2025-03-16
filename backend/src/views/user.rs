use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use proc_macros::GetStructFields;
use uuid::Uuid;
use crate::utils::traits_for_proc_macros::GetStructFields;

// maybe this shall be moved to a dedicated model_structs folder to allow the frontend to use them
// although for axum only this django like structure might suffice

// will this ever be used?

#[derive(Debug, Serialize, Deserialize, FromRow, GetStructFields)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, FromRow, Clone, Debug, GetStructFields)]
pub struct Username {
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserForRegister {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(FromRow, Debug, GetStructFields)]
pub struct FullUserForTest {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub password_encryption_salt: Uuid
}

// might be kind to let the user use either the email or the username
// #[derive(Deserialize)]
// pub struct UserForLogin {
//     pub email: String,
//     pub password: String
// }