use dotenv::dotenv;
use log::{error, info};
use serde::Deserialize;
use std::process::exit;

fn default_site_url() -> String {
    "https://cgu-odisha.ac.in/notice/".to_string()
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_site_url")]
    #[allow(dead_code)]
    pub site_url: String,
    pub db_connection_uri: String,
    pub database_name: String,
    pub collection_name: String,
    pub db_unique_field_name: String,
    pub db_unique_field_value: String,
    pub teloxide_token: String,
}

pub fn parse_env() -> Config {
    match dotenv() {
        Ok(_) => info!("loaded .env file."),
        Err(err) => {
            error!("Failed to load .env file, {}", err.to_string());
            exit(1);
        }
    }

    match envy::from_env::<Config>() {
        Ok(config) => return config,
        Err(err) => {
            error!("Failed to parse env vars, {}", err.to_string());
            exit(1);
        }
    }
}
