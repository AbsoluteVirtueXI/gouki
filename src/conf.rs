use crate::error::GoukiError;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use toml::de::Error;

const CONFIG_FILE: &str = "gouki.toml";

#[derive(Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: u16,
}

pub fn get_conf_from_file() -> Result<Config, GoukiError> {
    let contents = fs::read_to_string(CONFIG_FILE)
        .unwrap()
        .to_ascii_lowercase();
    let config: Config = toml::from_str(&contents).unwrap();
    Ok(config)
}
