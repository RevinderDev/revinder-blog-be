use axum::extract::rejection::JsonRejection;
use axum::{extract::FromRequest, http::StatusCode};

use axum::extract::Json;
use garde::Validate;

use crate::errors::{BoxedAppError, bad_request, custom};

#[derive(Debug)]
pub struct ValidatedJson<T>(pub T);

impl<T> std::ops::Deref for ValidatedJson<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    T: Validate<Context = ()>,
    S: Send + Sync,
{
    type Rejection = BoxedAppError;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<T>::from_request(req, state)
            .await
            .map_err(|rejection| match rejection {
                JsonRejection::MissingJsonContentType(_) => {
                    custom(StatusCode::UNSUPPORTED_MEDIA_TYPE, rejection.body_text())
                }
                _ => bad_request(rejection.body_text()),
            })?;

        payload
            .validate()
            .map_err(|errors| custom(StatusCode::UNPROCESSABLE_ENTITY, errors.to_string()))?;

        Ok(ValidatedJson(payload))
    }
}
