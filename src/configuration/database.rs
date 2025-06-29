use std::{fmt::Display, time::Duration};

use sea_orm::ConnectOptions;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use tracing_log::log::LevelFilter;

#[derive(Deserialize, Clone, Default)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: SecretString,
    #[serde(default = "default_ssl_mode")]
    pub ssl_mode: bool,
}

impl DatabaseSettings {
    pub fn build(&self) -> ConnectOptions {
        let mut opt = ConnectOptions::new(self.to_string());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false)
            .sqlx_logging_level(LevelFilter::Error)
            .set_schema_search_path("./schema")
            .to_owned()
    }
}

impl Display for DatabaseSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database
        )
    }
}

fn default_ssl_mode() -> bool {
    false
}
