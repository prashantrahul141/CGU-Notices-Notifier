use crate::structs;
use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use mongodb::Collection;

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

/// Gets metadata document using collection and not client.
/// # Arguments.
/// * `metadata_collection` - &mongodb::Collection<structs::DbMetaData>
pub async fn get_metadata_document_from_collection(
    metadata_collection: &mongodb::Collection<structs::DbMetaData>,
) -> structs::DbMetaData {
    metadata_collection
        .find_one(doc! { "data_id": "metadata" }, None)
        .await
        .expect("Failed to retrieve collection.")
        .expect("Failed to find metadata document in collection.")
}

/// adds user to notification subscribers' list.
/// # arguments
/// * `user_id` : &string - the user id which needs to be added.
/// * `metadata_collection` : &mongodb::collection<dbmetadata> - the metadata collection.
pub async fn add_user_to_subscribers(
    user_id: &String,
    metadata_collection: &Collection<structs::DbMetaData>,
) {
    let current_document = get_metadata_document_from_collection(&metadata_collection).await;
    let mut subscribed_users = current_document.subscribed_users;
    if !subscribed_users.contains(user_id) {
        subscribed_users.push(user_id.clone());

        let filter = doc! {"data_id" : "metadata"};
        let update = doc! {"$set": {"subscribed_users": &subscribed_users}};

        let _ = metadata_collection.update_one(filter, update, None).await;
    }
}

/// removes user to notification subscribers' list.
/// # arguments
/// * `user_id` : &string - the user id which needs to be removed.
/// * `collection` : &mongodb::collection<dbmetadata> - the metadata collection.
pub async fn remove_user_from_subscribers(
    user_id: &String,
    metadata_collection: &Collection<structs::DbMetaData>,
) {
    let current_document = get_metadata_document_from_collection(&metadata_collection).await;
    let mut subscribed_users = current_document.subscribed_users;

    subscribed_users.retain(|x| x.clone() != user_id.clone());

    let filter = doc! {"data_id" : "metadata"};
    let update = doc! {"$set": {"subscribed_users": &subscribed_users}};

    let _ = metadata_collection.update_one(filter, update, None).await;
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

/// takes db collection and entries,
/// adds the entries to collection.
pub async fn update_db_notices(
    db_col: &mongodb::Collection<structs::NoticeElement>,
    entries: &Vec<structs::NoticeElement>,
) {
    info!(
        "updating notice entries, adding {} new entries.",
        entries.len()
    );
    db_col
        .insert_many(entries, None)
        .await
        .expect("failed to add noticeelement entries to database.");
    info!("updated notices entries : {} elements", entries.len());
}

/// Takes latest hash and updates the metadata.
pub async fn update_latest_hash(
    latest_hash: &String,
    metadata_collection: &mongodb::Collection<structs::DbMetaData>,
) {
    info!("Updating latest hash.");
    let filter = doc! {"data_id" : "metadata"};
    let update = doc! {"$set": {"latest_hash": &latest_hash}};
    metadata_collection
        .update_one(filter, update, None)
        .await
        .expect("Failed to update latest_hash entry metadata.");
}
