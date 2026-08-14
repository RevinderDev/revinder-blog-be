use std::time::Duration;
use std::{str::FromStr, sync::Arc};

use axum::http::StatusCode;

use axum::{Router, routing::post};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::timeout::{RequestBodyTimeoutLayer, TimeoutLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing::info;
use tracing_subscriber::EnvFilter;

use revinder_blog_be::user::create_user;

use revinder_blog_be::config::{AppState, load_configuration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = load_configuration();
    let db_options = SqliteConnectOptions::from_str(&config.db.connection_string)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .busy_timeout(Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(db_options)
        .await
        .unwrap();
    let app_state = Arc::new(AppState { pool });
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("revinder_blog_be=debug,tower_http=info"))
                .unwrap(),
        )
        .init();
    let app = Router::new()
        .route("/", post(create_user))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(CatchPanicLayer::new())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(RequestBodyTimeoutLayer::new(Duration::from_secs(30)))
        .with_state(app_state);

    let port = config.app.port;
    let addr = format!("0.0.0.0:{port}");
    info!("Starting server on `{}`", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
