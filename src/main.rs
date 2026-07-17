use axum::http::StatusCode;
use axum::response::IntoResponse;
use figment::Figment;
use figment::providers::Env;
use log::info;

use axum::{Router, routing::post};
use garde::Validate;
use serde::Deserialize;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::EnvFilter;

use revinder_blog_be::common::{Email, Password};
use revinder_blog_be::validation::ValidatedJson;

#[derive(Deserialize, Debug, Validate)]
struct CreateUserRequest {
    #[garde(dive)]
    email: Email,

    #[garde(dive)]
    password: Password,
}

async fn create_user(
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> impl IntoResponse {
    info!("Received payload: {:#?}", payload);
    info!("Received payload: {:#?}", payload.email);
    info!("Received payload: {:#?}", payload.password);

    StatusCode::OK
}

#[derive(Deserialize, Debug)]
struct ServerConfiguration {
    port: u16,
}

#[derive(Deserialize, Debug)]
struct Configuration {
    app: ServerConfiguration,
}

fn load_configuration() -> Configuration {
    dotenvy::dotenv().ok();
    Figment::new()
        .merge(Env::raw().split("__"))
        .extract()
        .unwrap()
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

    let port = config.app.port;
    let addr = format!("0.0.0.0:{port}");
    info!("Starting server on `{}`", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
