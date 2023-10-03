use crate::structs;
use mongodb::{bson::doc, options::ClientOptions, Client};

///
pub async fn get_document(
    connection_uri: &String,
    database_name: &String,
    collection_name: &String,
) -> structs::DbCollectionType {
    info!("Creating db.");
    let client_options = ClientOptions::parse(connection_uri);
    let client = Client::with_options(client_options.await.unwrap())
        .expect("Failed to establish connection to the db.");
    let db_con = client.database(database_name);
    let data_col = db_con.collection::<structs::DbCollectionType>(collection_name);

    let filter = doc! { "data_id": "cgu-data-id"};
    info!("Finding document.");
    let found_document = data_col
        .find_one(Some(filter), None)
        .await
        .expect("Failed to establish connection to db.")
        .expect("Failed to establish connection to db.");

    found_document
}
