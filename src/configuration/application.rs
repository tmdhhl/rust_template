use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
}
