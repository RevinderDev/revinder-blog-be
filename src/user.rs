use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use log::info;

use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::config::AppState;
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
struct UserEntity {
    id: i64,
    email: String,
    password: String,
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> impl IntoResponse {
    let user: Option<UserEntity> = sqlx::query_as("SELECT id, email, password FROM users")
        .fetch_optional(&state.pool)
        .await
        .unwrap();

    if let Some(user) = user {
        info!("Fetched user {}", user.email);
    } else {
        info!("No user found");
    }
    info!("Received payload: {:#?}", payload);
    info!("Received payload: {:#?}", payload.email);
    info!("Received payload: {:#?}", payload.password);

    StatusCode::OK
}
