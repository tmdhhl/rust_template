use std::ops::Deref;

use serde::{Deserialize, Serialize};
use tracing::Level;
use tracing_appender::rolling::Rotation;

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

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TargetKind {
    #[default]
    Stdout,
    File,
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

pub fn default_level() -> Level {
    Level::INFO
}

pub fn default_rotation() -> Rotation {
    Rotation::DAILY
}
