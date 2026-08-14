use std::{borrow::Cow, error::Error, fmt};

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use tracing::error;

pub type AppResult<T> = Result<T, BoxedAppError>;
pub type BoxedAppError = Box<dyn AppError>;

#[derive(Debug, Clone)]
pub struct CustomApiError {
    status: StatusCode,
    detail: Cow<'static, str>,
}

fn json_error(detail: &str, status: StatusCode) -> Response {
    let json = json!({"errors": [{"detail": detail}]});
    (status, Json(json)).into_response()
}

pub fn custom(status: StatusCode, detail: impl Into<Cow<'static, str>>) -> BoxedAppError {
    Box::new(CustomApiError {
        status,
        detail: detail.into(),
    })
}

pub fn bad_request<S: ToString>(error: S) -> BoxedAppError {
    custom(StatusCode::BAD_REQUEST, error.to_string())
}

pub fn server_error<S: ToString>(error: S) -> BoxedAppError {
    error!(error = %error.to_string(), "Internal Server Error");
    custom(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

pub fn not_found() -> BoxedAppError {
    custom(StatusCode::NOT_FOUND, "Not found")
}

impl fmt::Display for CustomApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.detail.fmt(f)
    }
}

impl AppError for CustomApiError {
    fn response(&self) -> Response {
        json_error(&self.detail, self.status)
    }
}

pub trait AppError: Send + fmt::Display + fmt::Debug + 'static {
    fn response(&self) -> axum::response::Response;
}

impl AppError for BoxedAppError {
    fn response(&self) -> axum::response::Response {
        (**self).response()
    }
}

impl IntoResponse for BoxedAppError {
    fn into_response(self) -> axum::response::Response {
        self.response()
    }
}

impl<E: Error + Send + 'static> AppError for E {
    fn response(&self) -> axum::response::Response {
        error!(error = %self, "Internal Server Error");
        json_error("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<sqlx::Error> for BoxedAppError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed | sqlx::Error::Io(_) => {
                error!("SQLite storage error: {err}");
                custom(StatusCode::SERVICE_UNAVAILABLE, "Service Unavailable")
            }
            sqlx::Error::RowNotFound => not_found(),
            _ => Box::new(err),
        }
    }
}

pub trait SqlxResultExt<T> {
    fn on_unique_violation(self, message: impl Into<Cow<'static, str>>) -> AppResult<T>;
}

impl<T> SqlxResultExt<T> for Result<T, sqlx::Error> {
    fn on_unique_violation(self, message: impl Into<Cow<'static, str>>) -> AppResult<T> {
        self.map_err(|err| match &err {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                custom(StatusCode::CONFLICT, message)
            }
            _ => err.into(),
        })
    }
}
