use std::{default, fmt::format, ops::Deref, str::FromStr};

use anyhow::Result;
use serde::Deserialize;
use strum::{AsRefStr, Display, EnumString, IntoStaticStr};
use tracing_appender::rolling::Rotation;

#[derive(Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Settings<'a> {
    pub application_settings: ApplicationSettings,
    pub log_settings: LogSettings<'a>,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
}

#[derive(Deserialize)]
pub struct LogSettings<'a> {
    pub log_dir: &'a str,
    pub targets: Vec<Target<'a>>,
}

#[derive(Deserialize)]
pub struct Target<'a> {
    pub kind: TargetKind,
    #[serde(default = "default_filename")]
    pub filename: &'a str,
    pub level: &'a str,
    #[serde(default)]
    pub rotation: RotationKind,
}

fn default_filename() -> &'static str {
    "info.log"
}

#[derive(Deserialize)]
#[serde(rename_all(serialize = "snake_case"))]
pub enum TargetKind {
    Stdout,
    File,
}

#[derive(Deserialize, Default, Copy, Clone)]
#[serde(rename_all(serialize = "snake_case"))]
pub enum RotationKind {
    MINUTELY,
    HOURLY,
    #[default]
    DAILY,
    NEVER,
}

impl<'a> Settings<'a> {
    pub fn load() -> Result<Settings<'a>> {
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
