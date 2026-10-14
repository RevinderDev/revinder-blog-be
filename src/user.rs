use std::sync::Arc;

use argon2::password_hash::phc::SaltString;
use argon2::{Argon2, PasswordHasher};
use axum::Json;
use axum::extract::State;

use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::config::AppState;
use crate::errors::{AppResult, SqlxResultExt};
use crate::{
    common::{Email, Password},
    validation::ValidatedJson,
};

#[derive(Deserialize, Debug, Validate)]
pub struct CreateUserRequest {
    #[garde(dive)]
    email: Email,

    #[garde(dive)]
    password: Password,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct UserEntity {
    id: i64,
    email: Email,
    is_activated: bool,
}

// pub async fn get_users() {
//     let user: Option<UserEntity> = sqlx::query_as("SELECT id, email, password FROM users")
//         .fetch_optional(&state.pool)
//         .await
//         .unwrap();
//
//     if let Some(user) = user {
//         info!("Fetched user {:#?}", user.email);
//     } else {
//         info!("No user found");
//     }
// }

// TODO: Should have some tracing included.
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> AppResult<Json<UserEntity>> {
    let hashed_password = hash_password(payload.password).await.ok();
    let user: UserEntity = sqlx::query_as!(
        UserEntity,
        "INSERT INTO users (email, password_hash) VALUES (?, ?) RETURNING id, email, is_activated;",
        payload.email,
        hashed_password
    )
    .fetch_one(&state.pool)
    .await
    .on_unique_violation("A user with that email address already exists")?;
    Ok(Json(user))
}

async fn hash_password(password: Password) -> Result<String, argon2::password_hash::Error> {
    tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate();
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password_with_salt(password.as_bytes(), salt.as_bytes());
        password_hash.map(|hash| hash.to_string())
    })
    .await
    .expect("could not spawn_blocking for hash_password")
}
