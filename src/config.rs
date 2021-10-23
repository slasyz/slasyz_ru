use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub jwt: JWTConfig,
    pub toxic: ToxicConfig,
}

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub address: String,
}

#[derive(Serialize, Deserialize)]
pub struct JWTConfig {
    pub secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct ToxicConfig {
    pub url: String,
}

impl Config {
    pub fn new(path: &str) -> Result<Config, Box<dyn Error>> {
        let data = fs::read_to_string(path)?;
        let c: Config = serde_json::from_str(data.as_str())?;

        Ok(c)
    }
}
