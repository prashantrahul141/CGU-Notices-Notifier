use std::collections::HashMap;

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

pub fn get_table(site_text: &String) {
    let mut tables: HashMap<String, String> = HashMap::new();
    get_table_element(&site_text, &mut tables);
}

/// parses html text and find the required table element.
///
/// # Arguments
/// * `site_text` - Site html.
fn get_table_element(site_text: &String, tables_hash_map: &mut HashMap<String, String>) {
    info!("Parsing html document.");
    // let document = scraper::Html::parse_document(site_text).clone();
    // let table_selector = scraper::Selector::parse("div").unwrap();

    let site_dom = tl::parse(&site_text, tl::ParserOptions::default()).unwrap();
    let parser = site_dom.parser();
    let table_body_option = site_dom.query_selector("tr");

    info!("Searching for table in parsed document.");
    match table_body_option {
        Some(table_body_node) => {
            // looping through each tr tag in the table
            for table_element_option in table_body_node {
                // parsing the tr tag.
                let table_element_node_option = table_element_option.get(parser);

                match table_element_node_option {
                    Some(table_element_node) => {
                        println!("{:?}", table_element_node);
                    }
                    None => error!("Failed to find tr tags in the table."),
                }
            }
        }
        None => error!("Failed to find table in the document."),
    }
}
