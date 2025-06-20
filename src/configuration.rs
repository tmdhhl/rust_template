use anyhow::Result;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString, IntoStaticStr};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub application_settings: ApplicationSettings,
    pub log_settings: LogSettings,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct LogSettings {
    pub log_dir: String,
    pub targets: Vec<Target>,
}

#[derive(Serialize, Deserialize)]
pub struct Target {
    #[serde(default)]
    pub kind: TargetKind,
    #[serde(default = "default_filename")]
    pub filename: String,
    #[serde(default = "default_level")]
    pub level: String,
    #[serde(default)]
    pub rotation: RotationKind,
}

fn default_level() -> String {
    "info".into()
}

fn default_filename() -> String {
    "info.log".into()
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TargetKind {
    #[default]
    Stdout,
    File,
}

#[derive(Serialize, Deserialize, Default, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RotationKind {
    MINUTELY,
    HOURLY,
    #[default]
    DAILY,
    NEVER,
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
