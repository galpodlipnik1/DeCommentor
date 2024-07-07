use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename="indent")]
    pub indent_size: Option<u8>,
    #[serde(rename="removeComments")]
    pub remove_comments: Option<bool>,
    #[serde(rename="removeEmptyLines")]
    pub remove_empty_lines: Option<bool>,
    #[serde(rename="removeTrailingSpaces")]
    pub remove_trailing_spaces: Option<bool>,
    #[serde(rename="quoteStyle")]
    pub quote_style: Option<String>,
    #[serde(rename="bracketSpacing")]
    pub bracket_spacing: Option<bool>,
    #[serde(rename="ignore")]
    pub ignore_files: Option<Vec<String>>,
    pub path: Option<String>,
}

pub fn read_into_config(path: &str) -> Result<Config, String> {
    let config = fs::read_to_string(&path).map_err(|e| format!("Error reading config file: {:?}", e));

    let config = match serde_json::from_str(&config.unwrap()) {
        Ok(config) => config,
        Err(e) => {
            return Err(format!("Error parsing config file: {:?}", e));
        }
    };

    Ok(config)
}