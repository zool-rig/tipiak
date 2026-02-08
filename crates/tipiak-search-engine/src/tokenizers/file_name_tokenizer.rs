use std::{collections::HashSet, error::Error, path::Path};

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::token_utils::is_valid_token;

pub struct FileNameTokenizer;

impl Tokenizer for FileNameTokenizer {
    fn supports(&self, _path: &Path) -> bool {
        true
    }

    fn tokenize(&self, path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(path
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_lowercase()
            .replace(" ", "_")
            .split("_")
            .filter(is_valid_token)
            .map(|s| s.to_string())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect())
    }
}
