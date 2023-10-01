#![allow(dead_code)]
use mongodb::{options::ClientOptions, sync::Client};

///
pub fn mongo_client_init(connection_uri: &String) -> Client {
    let client_options = ClientOptions::parse(connection_uri);
    let client = Client::with_options(client_options.unwrap())
        .expect("Failed to establish connection to the db.");
    client
}
