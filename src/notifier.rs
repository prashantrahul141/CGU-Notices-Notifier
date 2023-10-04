use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};

use mongodb::bson::doc;
use tokio::time::sleep;

use crate::{db, site_scraper, structs};

// bot api url.
const BOT_URL: &str = "https://api.telegram.org/bot";

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

/// formats message string.
/// # Arguments.
/// * `entries` - &Vec<NoticeElements> - entries which needs to be formatterd.
/// # Returns
/// * Formatted String.
fn format_entries_into_message(entries: &Vec<structs::NoticeElement>) -> String {
    let mut results = Vec::<String>::new();
    for entry in entries {
        let formatted_entry_data = format!(
            " *{}* %0A%0A _{}_ %0A%0ADF : [Link]({})",
            entry.title, entry.date, entry.file_url
        );
        let intermediate = urlencoding::encode(&formatted_entry_data);
        results.push(intermediate.to_string());
    }

    // replaces `.` with `\\.` because telegram api says so.
    let mut result = results.join("%0A%0A");
    result = result.replace(".", "\\.");
    result
}

/// Send notifications to users.
/// # Arguments
/// *
async fn send_notifications(
    entries: &Vec<structs::NoticeElement>,
    users: &Vec<String>,
    bot_url_with_token: &String,
) {
    info!("Sending Notifications to {} Subscribers.", users.len());

    // looping through all subscribers.
    let mut i = 0;
    for user_chat_id in users {
        let request_url = format!(
            "{}{}{}{}{}{}",
            bot_url_with_token,
            String::from("/sendmessage?chat_id="),
            user_chat_id,
            "&text=",
            format_entries_into_message(&entries),
            "&parse_mode=Markdown"
        );

        // sleeping to avoid rate limiting.
        if i == 10 {
            i = 0;
            sleep(Duration::from_secs(1)).await;
        }
        i += 1;

        let response = reqwest::get(&request_url).await;
        dbg!(request_url);

        match response {
            Ok(result) => {
                dbg!(&result);
                if result.status() == 429 {
                    warn!("Recieved 429 from telegram servers.");
                    warn!("Sleeping for 10 seconds.");
                    sleep(Duration::from_secs(10)).await;
                };
            }

            Err(err) => {
                info!("Error sending messages. {}", err);
            }
        }
    }
}

/// notfiy loop
/// # Arguments.
/// * `db_client`: &mongodb::Client - Database client.
/// * `database_name` : &String - Database name.
/// * `site_url` : &String - Site's url to parse.
///
pub async fn notify_loop(
    db_client: &mongodb::Client,
    database_name: &String,
    site_url: &String,
    bot_token: &String,
) {
    info!("Running notify loop.");
    let bot_url_with_token = String::from(BOT_URL) + bot_token;
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

            // sending notificaitons to subscribers.
            send_notifications(
                &new_notice_elements,
                &metadata_document.subscribed_users,
                &bot_url_with_token,
            )
            .await;
        }

        info!("Sleeping notify loop.");
        sleep(Duration::from_secs(300)).await;
    }
}

// https://api.telegram.org/bot6385052871:AAH1lyZ4zvWxqHDb0UC3yTvvq32JAQkNQmY/sendmessage?chat_id=1298091653&text=%2AM\\.%20Sc%20\\%28CSE-%203rd%20Sem\\.\\%29)%2A%250A_September%2025%2C%202023_%250APDF%20%3A%20%5BLink%5D\\%28https%3A%2F%2Fcgu-odisha\\.ac\\.in%2Fwp-content%2Fuploads%2F2023%2F09%2F3rd-M\\.Sc-CC\\.pdf\\%29)%0A%0A%2AM\\.%20Tech%20\\%28CSE-1st%20sem\\.\\%29)%2A%250A_September%2025%2C%202023_%250APDF%20%3A%20%5BLink%5D\\%28https%3A%2F%2Fcgu-odisha\\.ac\\.in%2Fwp-content%2Fuploads%2F2023%2F09%2F1st-M\\.Tech_\\.pdf\\%29)&parse_mode=MarkdownV2
