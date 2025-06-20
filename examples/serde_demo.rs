use serde::{Deserialize, Serialize};

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
    #[serde(default = "default_filename")]
    pub filename: String,
    #[serde(default = "default_level")]
    pub level: String,
    #[serde(default="default_rotation")]
    #[serde(with = "RotationDef")]
    pub rotation: Rotation,
}

fn default_rotation()-> Rotation {
    Rotation::DAILY
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
#[serde(remote = "Rotation")]
#[serde(rename_all = "lowercase")]
pub enum RotationDef {
    MINUTELY,
    HOURLY,
    #[default]
    DAILY,
    NEVER,
}

fn main() {}
