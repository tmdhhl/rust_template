use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use strum::{AsRefStr, Display, EnumString, IntoStaticStr};
use tracing::Level;
use tracing_appender::rolling::Rotation;

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
    #[serde(default = "FilenameString::default_filename")]
    pub filename: FilenameString,
    #[serde(with = "LevelDef")]
    #[serde(default = "default_level")]
    pub level: Level,
    #[serde(with = "RotationDef")]
    #[serde(default = "default_rotation")]
    pub rotation: Rotation,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(remote = "Level")]
#[serde(rename_all = "lowercase")]

pub enum LevelDef {
    TRACE,
    DEBUG,
    #[default]
    INFO,
    WARN,
    ERROR,
}

#[derive(Serialize, Deserialize)]
pub struct FilenameString(String);
impl FilenameString {
    fn default_filename() -> Self {
        FilenameString("info.log".into())
    }
}

impl Deref for FilenameString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TargetKind {
    #[default]
    Stdout,
    File,
}

#[derive(Serialize, Deserialize, Default, Copy, Clone)]
#[serde(remote = "Rotation")]
#[serde(rename_all = "lowercase")]
pub enum RotationDef {
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

pub fn default_level() -> Level {
    Level::INFO
}

pub fn default_rotation() -> Rotation {
    Rotation::DAILY
}
