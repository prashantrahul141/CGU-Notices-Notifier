use crate::structs::{self};
use mongodb::bson::doc;
use mongodb::options::ClientOptions;

/// Gets mongodb client.
/// # Arguements.
/// * `connection_uri` : &String - Connection url of mongodb instance.
pub async fn get_client(connection_uri: &String) -> mongodb::Client {
    let client_options = ClientOptions::parse(connection_uri);
    let client = mongodb::Client::with_options(client_options.await.unwrap());
    client.unwrap()
}

/// Gets the mongodb collection containing metadata document.
/// # Arguments.
/// * `db_client` : &mongodb::Client
/// * `database_name` : &String - The database in which collection
/// needs to be searched.
pub fn get_metadata_collection(
    db_client: &mongodb::Client,
    database_name: &String,
) -> mongodb::Collection<structs::DbMetaData> {
    let db_con = db_client.database(&database_name);
    db_con.collection("metadata-col")
}

/// Gets the mongodb document containig metadata.
/// # Arguments.
/// * `db_client` : &mongodb::Client
/// * `database_name` : &String - The daatabase in which docuement
/// needs to be searched.
pub async fn get_metadata_document(
    db_client: &mongodb::Client,
    database_name: &String,
) -> structs::DbMetaData {
    let collection = get_metadata_collection(&db_client, &database_name);
    collection
        .find_one(doc! { "data_id": "metadata" }, None)
        .await
        .expect("Failed to retrieve collection.")
        .expect("Failed to find metadata document in collection.")
}

/// Gets the mongodb collection containing notices documents.
/// # Arguments.
/// * `db_client` : &mongodb::Client
/// * `database_name` : &String - The database in which collection
/// needs to be searched.
pub fn get_notices_collection(
    db_client: &mongodb::Client,
    database_name: &String,
) -> mongodb::Collection<structs::NoticeElement> {
    let db_con = db_client.database(&database_name);
    db_con.collection("all-notices-col")
}
