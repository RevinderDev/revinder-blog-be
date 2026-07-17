use axum::http::StatusCode;
use axum::response::IntoResponse;
use log::info;

use garde::Validate;
use serde::Deserialize;

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

pub async fn create_user(
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> impl IntoResponse {
    info!("Received payload: {:#?}", payload);
    info!("Received payload: {:#?}", payload.email);
    info!("Received payload: {:#?}", payload.password);

    StatusCode::OK
}
