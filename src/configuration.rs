use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::configuration::environment::Environment;

pub mod application;
pub mod environment;
pub mod logging;

pub use application::ApplicationSettings;
pub use logging::LogSettings;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub application_settings: ApplicationSettings,
    pub log_settings: LogSettings,
}

impl Settings {
    pub fn load() -> Result<Settings> {
        let current_dir = std::env::current_dir()?;
        let configuration = current_dir.join("configuration");

        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .map_or(Environment::Local, |env| env.parse().unwrap_or_default());

        let environment_filename = format!("{}.yaml", environment);

        let settings = config::Config::builder()
            .add_source(config::File::from(configuration.join("base.yaml")))
            .add_source(config::File::from(configuration.join(environment_filename)))
            .add_source(
                config::Environment::with_prefix("APP")
                    .separator("__")
                    .prefix_separator("_"),
            )
            .build()?;
        settings.try_deserialize().map_err(|e| anyhow::anyhow!(e))
    }
}
