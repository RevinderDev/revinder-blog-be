use figment::Figment;
use figment::providers::Env;

use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize, Debug)]
pub struct DatabaseConfiguration {
    pub connection_string: String,
    // NOTE: Don't like how I need DATABASE_URL and
    // db__connection_string envs. Fix it later.
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
