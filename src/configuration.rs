use anyhow::Result;
use serde::Deserialize;
use strum::{AsRefStr, Display, EnumString, IntoStaticStr};

#[derive(Deserialize)]
pub struct Settings {
    pub application_settings: ApplicationSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
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

#[derive(IntoStaticStr, AsRefStr, EnumString, Default, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Environment {
    #[default]
    Local,
    Production,
}
