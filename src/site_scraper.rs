use scraper::ElementRef;

/// requests the given url and returns the html.
///
/// # Arguments
/// * `site` - Site url in String.
///
/// # Output
/// * `Result<String, Err>` - output.
pub async fn get_site_html(site: &String) -> Result<String, reqwest::Error> {
    info!("GET request to : {}", &site);
    let response_result = reqwest::get(site).await;
    match response_result {
        Ok(response_body) => {
            info!("Got response from {} successfully.", site);
            info!("Parsing response to text.");
            let response_text_result = response_body.text().await;
            match response_text_result {
                Ok(response_text) => {
                    info!("successfully parsed response text.");
                    return Ok(response_text);
                }
                Err(err) => {
                    error!("Failed to parse response text.");
                    return Err(err);
                }
            }
        }

        Err(err) => {
            error!("Failed to reach server.");
            return Err(err);
        }
    }
}

/// parses site text into html tree.
///
/// # Arguments
/// * `site_text` - Site html.

pub fn get_notices_table(site_text: &String) -> Option<ElementRef> {
    info!("Parsing html document.");
    let document = scraper::Html::parse_document(site_text);
    let table_selector = scraper::Selector::parse(".table").unwrap();

    info!("Searching for table in parsed document.");
    let selected_table = document.select(&table_selector).next();

    match selected_table {
        Some(table) => {
            info!("successfully found table in parsed document.");
            return Some(table);
        }
        None => {
            error!("Could not found table in parsed document.");
            return None;
        }
    }
}
