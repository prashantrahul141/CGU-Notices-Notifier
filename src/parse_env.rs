use serde::Deserialize;

fn default_site_url() -> String {
    "https://cgu-odisha.ac.in/notice/".to_string()
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_site_url")]
    site_url: String,
}

pub fn parse_env() {
    match envy::from_env::<Config>() {
        Ok(config) => println!("{:#?}", config),
        Err(err) => panic!(
            "{}",
            format!("Could not parse env vars, {}.", err.to_string())
        ),
    }
}
