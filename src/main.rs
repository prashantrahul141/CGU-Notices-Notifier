mod parse_env;
mod site_scraper;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Starting up");
    let env_vars: parse_env::Config = parse_env::parse_env();
    let site_html_result = site_scraper::get_site_html(&env_vars.site_url).await;
}
