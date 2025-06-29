use secrecy::SecretString;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    pub port: u16,
    pub pdd: PddSettings,
}

#[derive(Deserialize, Clone)]
pub struct PddSettings {
    pub client_id: SecretString,
    pub client_secret: SecretString,
    pub pid: SecretString,
    pub domain: String,
    pub api_good_search: String,
    pub api_gen_short_url: String,
}
