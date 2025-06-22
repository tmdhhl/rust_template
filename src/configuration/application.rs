use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ApplicationSettings {
    pub port: u16,
}
