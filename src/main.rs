mod bot;
mod db;
mod notifier;
mod parse_env;
mod site_scraper;
mod structs;
mod utils;

#[macro_use]
extern crate log;
extern crate chrono;
extern crate urlencoding;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting up");

    let env_vars: parse_env::Config = parse_env::parse_env();
    let bot_loop = async {
        bot::run(
            &env_vars.teloxide_token,
            &env_vars.db_connection_uri,
            &env_vars.database_name,
        )
        .await;
    };

    let notify_loop = async {
        notifier::notify_loop(
            &env_vars.db_connection_uri,
            &env_vars.database_name,
            &env_vars.site_url,
            &env_vars.teloxide_token,
        )
        .await;
    };

    tokio::join!(bot_loop, notify_loop);
}
