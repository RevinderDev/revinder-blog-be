use log::info;
use std::net::SocketAddr;

use axum::{Router, extract::Json, routing::post};
use serde::Deserialize;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use std::env;
use std::error::Error;

#[derive(Deserialize)]
struct CreateUserRequest {
    email: String,
    password: String,
}

async fn create_user(Json(payload): Json<CreateUserRequest>) {}

#[derive(Deserialize)]
struct AppConfig {
    port: u16,
}

fn load_env() -> AppConfig {
    dotenvy::dotenv().ok();
    envy::prefixed("APP_").from_env::<AppConfig>().unwrap()
}

#[tokio::main]
async fn main() {
    let config = load_env();
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
}
