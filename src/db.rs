use crate::structs;
use mongodb::{bson::doc, options::ClientOptions, Client};

pub async fn get_client(connection_uri: &String) -> Client {
    let client_options = ClientOptions::parse(connection_uri);
    let client = Client::with_options(client_options.await.unwrap());
    client.unwrap()
}

pub async fn get_document(
    client: &Client,
    database_name: &String,
    collection_name: &String,
) -> structs::DbCollectionType {
    info!("Getting document.");
    let db_con = client.database(database_name);
    let data_col = db_con.collection::<structs::DbCollectionType>(collection_name);

    let filter = doc! { "data_id": "cgu-data-id"};
    info!("Finding document.");

    let found_document = data_col
        .find_one(Some(filter), None)
        .await
        .expect("failed to parse collection.")
        .expect("failed to find exact document in collection.");

    found_document
}
