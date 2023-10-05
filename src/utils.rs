use crate::structs;

pub fn sanitize(entry: &String) -> String {
    let result = entry.replace("&#038;", "%26");
    result
}

/// formats message string.
/// # Arguments.
/// * `entries` - &Vec<NoticeElements> - entries which needs to be formatterd.
/// # Returns
/// * Formatted String.
pub fn format_entries_into_message(entries: &Vec<structs::NoticeElement>) -> String {
    let mut results = Vec::<String>::new();
    for entry in entries {
        let formatted_entry_data = format!(
            "{}%0A{}%0ALink : {}",
            sanitize(&entry.title),
            sanitize(&entry.date),
            sanitize(&entry.file_url)
        );
        results.push(formatted_entry_data.to_string());
    }

    // add new line after every entry.
    results.join("%0A%0A")
}
