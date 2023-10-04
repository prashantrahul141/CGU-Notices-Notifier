use crate::structs::NoticeElement;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use tl::{Node, Parser};

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
                    Ok(response_text)
                }
                Err(err) => {
                    error!("Failed to parse response text.");
                    Err(err)
                }
            }
        }

        Err(err) => {
            error!("Failed to reach server.");
            Err(err)
        }
    }
}

/// takes in site html in text and return vec of NoticeElement.
///
/// # Arguments
/// * `site_text` : &String - site html in text.
///
/// # Returns
/// * `table`  : vec<u64, NoticeElement>
pub fn get_notice_elements(site_text: &String) -> Vec<NoticeElement> {
    let mut tables_vec: Vec<NoticeElement> = Vec::new();
    get_table_element(&site_text, &mut tables_vec);
    tables_vec
}

/// parses tr element of each cell and returns a NoticeElement struct.
///
/// # Arguments
/// * `tr_element` : &Node - tr element node.
/// * `parser` : &Parser - dom parser.
fn parse_tr_to_notice(tr_element: &Node, parser: &Parser) -> Option<NoticeElement> {
    trace!("parsing a tr tag.");
    let mut default_hasher = DefaultHasher::new();
    match tr_element.children() {
        Some(table_cell_element) => {
            trace!("extracing info from cell data.");

            // all elements in the indiviual tr tag.
            let cell_elements = table_cell_element.all(parser);

            // extracing each elements.
            let title = String::from_utf8(cell_elements[5].inner_text(parser).as_bytes().to_vec())
                .unwrap_or("[Title]".to_string());
            let date = String::from_utf8(cell_elements[8].inner_text(parser).as_bytes().to_vec())
                .unwrap_or("[Date]".to_string());
            let file_url = match cell_elements[12].as_tag() {
                Some(tag) => {
                    let attrs = tag.attributes();
                    let href_attr = attrs.get("href").flatten();
                    match href_attr {
                        Some(stri) => stri.as_utf8_str().to_string(),
                        None => "None".to_string(),
                    }
                }
                None => "None".to_string(),
            };
            trace!("hashing.");
            // hashing is done using the notice's title and date
            // concats the title and the date and hash the resultant.
            let hash_string = date.clone() + &title;
            hash_string.hash(&mut default_hasher);
            let hash = default_hasher.finish().to_string();

            trace!("instantiating NoticeElement.");
            // instantiate and return struct NoticeElement.
            Some(NoticeElement {
                title,
                date,
                file_url,
                hash,
            })
        }

        None => {
            error!("Could not parse table cells.");
            None
        }
    }
}

/// parses html text and find the required table element.
///
/// # Arguments
/// * `site_text` - Site html.
fn get_table_element(site_text: &String, tables_hash_map: &mut Vec<NoticeElement>) {
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
            info!("looping through all tr tags.");
            for (current_tr_index, table_element_option) in table_body_node.enumerate() {
                // skipping the first tr because it contains table headers.
                if current_tr_index != 0 {
                    let tr_element_option = table_element_option.get(parser);

                    match tr_element_option {
                        Some(tr_element) => match parse_tr_to_notice(&tr_element, &parser) {
                            Some(parsed_notice) => {
                                trace!("Saving NoticeElement to vec.");
                                tables_hash_map.push(parsed_notice);
                            }
                            None => error!("Failed to create NoticeElement."),
                        },
                        None => error!("Failed to find tr tags in the table."),
                    }
                }
            }
        }
        None => error!("Failed to find table in the document."),
    }
}
