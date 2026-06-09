use std::{error::Error, path::Path};

use libzim_rs::parse_zim;

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::fs_utils::is_zim_file;
use crate::utils::token_utils::{is_valid_token, sanitize_word};

const METADATA_KEYS: &'static [&str] = &[
    "Creator",
    "Description",
    "Name",
    "Tags",
    "Title",
];

pub struct ZimTokenizer;

impl Tokenizer for ZimTokenizer {
    fn supports(&self, path: &Path) -> bool {
        is_zim_file(path)
    }

    fn tokenize(&self, path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
        let mut tokens: Vec<String> = Vec::new();
        
        let zim_file = parse_zim(&path.display().to_string())?;

        for key in zim_file.metadata_keys() {
            if !METADATA_KEYS.contains(&key.as_str()) {
                continue;
            }

            match zim_file.get_metadata_str(&key) {
                Some(value) => {
                    println!("{:?}", value);
                    tokens.extend(
                    value
                    .split_whitespace()
                    .map(|t| t.split(";"))
                    .flatten()
                    .filter(|t| !t.starts_with("_"))
                    .map(|t| t.split("_"))
                    .flatten()
                    .map(|t| t.split("-"))
                    .flatten()
                    .filter(is_valid_token)
                    .map(sanitize_word)
                )},
                None => continue
            }
        }

        Ok(tokens)
    }
}
