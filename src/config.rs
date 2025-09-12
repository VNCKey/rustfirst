// src/config.rs
use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub namespace: String,
    pub name: String,
    pub user: String,
    pub pass: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseConfig,
}

pub fn load_config() -> Result<Settings, config::ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name("config/default"))
        .add_source(File::with_name("config/production").required(false))
        .add_source(config::Environment::default().separator("__"))
        .build()?;

    settings.try_deserialize()
}
