use axum::http::StatusCode;
use axum::response::IntoResponse;
use log::info;

use axum::{Router, extract::Json, routing::post};
use axum_valid::Garde;
use garde::Validate;
use serde::Deserialize;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[derive(Deserialize, Debug, Validate)]
struct CreateUserRequest {
    #[garde(email)]
    email: String,

    #[garde(length(min = 15))]
    password: String,
}

async fn create_user(Garde(payload): Garde<Json<CreateUserRequest>>) -> impl IntoResponse {
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
                .or_else(|_| EnvFilter::try_new("revinder_blog_be=debug,tower_http=warn"))
                .unwrap(),
        )
        .init();
    let app = Router::new()
        .route("/", post(create_user))
        .layer(TraceLayer::new_for_http());

    let port = config.port;
    let addr = format!("0.0.0.0:{port}");
    info!("Starting server on `{}`", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
