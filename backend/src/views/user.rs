use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, prelude::FromRow};
use proc_macros::GetStructFields;
use uuid::Uuid;
use crate::utils::traits_for_proc_macros::GetStructFields;

// maybe this shall be moved to a dedicated model_structs folder to allow the frontend to use them
// although for axum only this django like structure might suffice

// will this ever be used?

pub trait GettableUser: for<'r> FromRow<'r, PgRow> + Unpin + Send + GetStructFields {}

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
// but that would be a lot of work

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserForLogin {
    pub email: String,
    pub password: String
}

#[derive(Debug, FromRow, GetStructFields)]
pub struct UserForValidation {
    pub id: i64,
    pub password: String,
    pub password_encryption_salt: Uuid,
    pub token_encryption_salt: Uuid
}

#[derive(Debug, FromRow, GetStructFields)]
pub struct UserForAuth {
    pub id: i64,
    pub token_encryption_salt: Uuid
}

impl GettableUser for UserForAuth {}
impl GettableUser for User {}