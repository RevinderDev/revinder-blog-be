use axum::extract::rejection::JsonRejection;
use axum::response::{IntoResponse, Response};
use axum::{extract::FromRequest, http::StatusCode};
use log::info;

use axum::{Router, extract::Json, routing::post};
use garde::Validate;
use serde::Deserialize;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::EnvFilter;

#[derive(Debug)]
struct ValidatedJson<T>(pub T);

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

#[derive(Deserialize, Validate, Debug)]
#[garde(transparent)]
struct UserEmail(#[garde(email)] String);

#[derive(Deserialize, Validate, Debug)]
#[garde(transparent)]
struct UserPassword(#[garde(length(min = 15))] String);

#[derive(Deserialize, Debug, Validate)]
struct CreateUserRequest {
    #[garde(dive)]
    email: UserEmail,

    #[garde(dive)]
    password: UserPassword,
}

async fn create_user(
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> impl IntoResponse {
    info!("Received payload: {:#?}", payload);
    info!("Received payload: {:#?}", payload.email);
    info!("Received payload: {:#?}", payload.password);

    StatusCode::OK
}

#[derive(Deserialize)]
struct AppConfig {
    port: u16,
}

fn load_configuration() -> AppConfig {
    dotenvy::dotenv().ok();
    envy::prefixed("APP_").from_env::<AppConfig>().unwrap()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = load_configuration();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("revinder_blog_be=debug,tower_http=info"))
                .unwrap(),
        )
        .init();
    let app = Router::new().route("/", post(create_user)).layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO)),
    );

    let port = config.port;
    let addr = format!("0.0.0.0:{port}");
    info!("Starting server on `{}`", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
