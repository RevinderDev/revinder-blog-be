use axum::extract::rejection::JsonRejection;
use axum::response::{IntoResponse, Response};
use axum::{extract::FromRequest, http::StatusCode};

use axum::extract::Json;
use garde::Validate;

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
    type Rejection = Response;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(payload) = match Json::<T>::from_request(req, state).await {
            Ok(value) => value,
            Err(rejection) => {
                let status = match rejection {
                    JsonRejection::JsonDataError(_)
                    | JsonRejection::JsonSyntaxError(_)
                    | JsonRejection::BytesRejection(_) => StatusCode::BAD_REQUEST,
                    JsonRejection::MissingJsonContentType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };

                let json_error = serde_json::json!({
                    "error": "Parsing Error",
                    "message": rejection.body_text()
                });
                return Err((status, Json(json_error)).into_response());
            }
        };

        if let Err(errors) = payload.validate() {
            let json_error = serde_json::json!({
                "error": "ValidationError",
                "details": errors
            });

            return Err((StatusCode::UNPROCESSABLE_ENTITY, Json(json_error)).into_response());
        }

        Ok(ValidatedJson(payload))
    }
}
