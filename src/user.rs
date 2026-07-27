use std::sync::Arc;

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
    password: Password,
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
    let user: UserEntity = sqlx::query_as!(
        UserEntity,
        "INSERT INTO users (email, password) VALUES (?, ?) RETURNING *;",
        payload.email,
        payload.password
    )
    .fetch_one(&state.pool)
    .await
    .on_unique_violation("A user with that email address already exists")?;
    Ok(Json(user))
}
