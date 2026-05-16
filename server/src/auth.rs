use std::fmt;

use argon2::{Argon2, PasswordVerifier, PasswordHash};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use thiserror::Error;

use crate::schema::accounts;

thread_local! {
    static ARGON2: Argon2<'static> = Argon2::default();
}

pub async fn authenticate(
    db: &DatabaseConnection,
    username: String,
    password: String,
) -> Result<accounts::Model, AuthError> {
    let account = accounts::Entity::find()
        .filter(accounts::Column::Username.eq(username))
        .one(db)
        .await?
        .ok_or(AuthError::AccountNotFound)?;
    let password_hash = PasswordHash::new(&account.password_hash).inspect_err(|e| {
        tracing::error!("Failed to parse password hash for account: {}", e);
    })?;

    let is_password_valid = ARGON2.with(|argon2| {
        argon2.verify_password(password.as_bytes(), &password_hash).is_ok()
    });
    if !is_password_valid {
        return Err(AuthError::IncorrectPassword);
    }

    Ok(account)
}

#[derive(Debug, Error)]
pub enum AuthError {
    AccountNotFound,
    HashingError(argon2::password_hash::Error),
    DatabaseError(#[from] sea_orm::DbErr),
    IncorrectPassword,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::AccountNotFound => write!(f, "Account not found"),
            AuthError::HashingError(e) => write!(f, "Hashing error: {}", e),
            AuthError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AuthError::IncorrectPassword => write!(f, "Incorrect password"),
        }
    }
}

impl From<argon2::password_hash::Error> for AuthError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AuthError::HashingError(err)
    }
}