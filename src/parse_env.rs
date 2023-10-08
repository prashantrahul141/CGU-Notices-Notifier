use dotenv::dotenv;
use log::{error, info};
use serde::Deserialize;

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
    pub notices_collection_name: String,
    pub metadata_collection_name: String,
    pub teloxide_token: String,
}

pub fn parse_env() -> Config {
    if cfg!(debug_assertions) {
        info!("In debug mode loading .env file.");
        match dotenv() {
            Ok(_) => info!("loaded .env file."),
            Err(err) => {
                error!("Failed to load .env file, {}", err.to_string());
                panic!();
            }
        }
    }

    match envy::from_env::<Config>() {
        Ok(config) => return config,
        Err(err) => {
            error!("Failed to parse env vars, {}", err.to_string());
            panic!();
        }
    }
}
