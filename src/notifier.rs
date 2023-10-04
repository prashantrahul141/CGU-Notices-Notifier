use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};

use mongodb::bson::doc;
use tokio::time::sleep;

use crate::{db, site_scraper, structs};

/// Takes db collection and entries,
/// adds the entries to collection.
async fn update_db_notices(
    db_col: &mongodb::Collection<structs::NoticeElement>,
    entries: &Vec<structs::NoticeElement>,
) {
    info!(
        "Updating notice entries, adding {} new entries.",
        entries.len()
    );
    db_col
        .insert_many(entries, None)
        .await
        .expect("Failed to add NoticeElement entries to database.");
    info!("Updated notices entries : {} Elements", entries.len());
}

/// Takes latest hash and updates the metadata.
async fn update_latest_hash(
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

/// notfiy loop
/// # Arguments.
/// * `db_client`: &mongodb::Client - Database client.
/// * `database_name` : &String - Database name.
/// * `site_url` : &String - Site's url to parse.
///
pub async fn notify_loop(db_client: &mongodb::Client, database_name: &String, site_url: &String) {
    info!("Running notify loop.");
    loop {
        // metadata document.
        let metadata_document = db::get_metadata_document(&db_client, &database_name).await;
        let latest_hash = metadata_document.latest_hash;

        // all notices collection.
        let notices_collection = db::get_notices_collection(&db_client, &database_name);

        // metadata collection.
        let metadata_collection = db::get_metadata_collection(&db_client, &database_name);

        // requesting site and parsing it.
        let site_html = site_scraper::get_site_html(site_url).await.unwrap();
        let notice_elements = site_scraper::get_notice_elements(&site_html);

        let mut new_notice_elements = Vec::<structs::NoticeElement>::new();

        // looping through all notice elements scraped from the site.
        for notice_element in notice_elements {
            // calculate hash of notice_element.
            let mut default_hasher = DefaultHasher::new();
            let hash_string = notice_element.date.clone() + &notice_element.title;
            hash_string.hash(&mut default_hasher);
            let current_hash = default_hasher.finish().to_string();

            if current_hash == latest_hash {
                trace!("Stopping entries loop.");
                break;
            } else {
                new_notice_elements.push(notice_element);
            }
        }

        // if len of new elements added is > 0.
        if new_notice_elements.len() > 0 as usize {
            // add new entries to collection.
            update_db_notices(&notices_collection, &new_notice_elements).await;

            // update latest_hash.
            update_latest_hash(&new_notice_elements[0].hash, &metadata_collection).await;
        }

        info!("Sleeping notify loop.");
        sleep(Duration::from_secs(10)).await;
    }
}
