use std::{collections::HashSet, error::Error, path::Path};

use libzim_rs::parse_zim;

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::fs_utils::is_file_type;
use crate::utils::token_utils::tokenize_string;

const METADATA_KEYS: &[&str] = &["Creator", "Description", "Name", "Tags", "Title"];

pub struct ZimTokenizer;

impl Tokenizer for ZimTokenizer {
    fn supports(&self, path: &Path) -> bool {
        is_file_type(path, vec!["zim"])
    }

    fn tokenize(&self, path: &Path, _root_dir: &Path) -> Result<HashSet<String>, Box<dyn Error>> {
        let mut tokens: HashSet<String> = HashSet::new();

        let zim_file = parse_zim(&path.display().to_string())?;

        for key in zim_file.metadata_keys() {
            if !METADATA_KEYS.contains(&key.as_str()) {
                continue;
            }

            match zim_file.get_metadata_str(&key) {
                Some(value) => tokens.extend(tokenize_string(value)),
                None => continue,
            }
        }

        Ok(tokens)
    }
}
