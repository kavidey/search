use regex::Regex;
use once_cell::sync::Lazy;
use std::path::MAIN_SEPARATOR;

pub fn split_filename(string: &str) -> Vec<String> {
    static CAMEL_CASE_SPLIT: Lazy<Regex> = Lazy::new(|| Regex::new(r"([a-z&])([&A-Z0-9])|([^ ])([A-Z][a-z])|([0-9])([a-z])").unwrap());

    static DELIMITERS_SPLIT: Lazy<Regex> = Lazy::new(|| {
        let delimiters = [" ", ".", "_", "-"];
        let delimiters_split_str = delimiters
            .iter()
            .map(|delim| regex::escape(delim))
            .collect::<Vec<String>>()
            .join("|");
        Regex::new(&delimiters_split_str).unwrap()
    });

    let camel_case_split = CAMEL_CASE_SPLIT.split(string);

    let mut result = Vec::new();
    for part in camel_case_split {
        for split_part in DELIMITERS_SPLIT.split(part) {
            if !split_part.is_empty() {
                result.push(split_part.to_string());
            }
        }
    }

    result
}

pub fn split_path(path: &str) -> Vec<String> {
    // Split the path into segments using the main separator (`/` or `\`).
    let segments: Vec<&str> = path.split(MAIN_SEPARATOR).collect();

    // Split each segment into words using `split_file_name` and flatten the results.
    segments
        .into_iter()
        .flat_map(|segment| split_filename(segment))
        .collect()
}