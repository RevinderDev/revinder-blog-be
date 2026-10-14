use figment::Figment;
use figment::providers::Env;

use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize, Debug)]
pub struct DatabaseConfiguration {
    pub connection_string: String,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfiguration {
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub app: ServerConfiguration,
    pub db: DatabaseConfiguration,
}

pub struct AppState {
    pub pool: SqlitePool,
}

pub fn load_configuration() -> Configuration {
    dotenvy::dotenv().ok();
    Figment::new()
        .merge(Env::raw().split("__"))
        .extract()
        .unwrap()
}
