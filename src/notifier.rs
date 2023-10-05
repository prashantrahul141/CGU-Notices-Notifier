use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};

use tokio::time::sleep;

use crate::{db, site_scraper, structs, utils};

// bot api url.
const BOT_URL: &str = "https://api.telegram.org/bot";

/// Send messages to subscribed users.
/// # Arguments
/// * `entries` : &Vec<structs::NoticeElement> - notices list.
/// * `users` : &Vec<structs::NoticeElement> - subscribed users.
/// * `bot_url_with_token` : &String - Bot connection url.
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
            utils::format_entries_into_message(&entries),
            "&disable_web_page_preview=true"
        );

        // sleeping to avoid rate limiting.
        if i == 10 {
            i = 0;
            sleep(Duration::from_secs(1)).await;
        }
        i += 1;
        let response = reqwest::get(&request_url).await;

        match response {
            Ok(result) => {
                if result.status() == 429 {
                    warn!("Recieved 429 from telegram servers.");
                    warn!("Sleeping for 10 seconds.");
                    sleep(Duration::from_secs(10)).await;
                };
                // debug telegram's response.
                debug!("{}", result.text().await.unwrap());
            }

            Err(err) => {
                error!("Error sending messages. {}", err);
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
    db_connection_uri: &String,
    database_name: &String,
    site_url: &String,
    bot_token: &String,
) {
    info!("Running notify loop.");
    let bot_url_with_token = String::from(BOT_URL) + bot_token;
    let db_client = db::get_client(&db_connection_uri).await;

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

        // dbg!(&new_notice_elements);

        // if len of new elements added is > 0.
        if new_notice_elements.len() > 0 as usize {
            // add new entries to collection.
            db::update_db_notices(&notices_collection, &new_notice_elements).await;

            // update latest_hash.
            db::update_latest_hash(&new_notice_elements[0].hash, &metadata_collection).await;

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
