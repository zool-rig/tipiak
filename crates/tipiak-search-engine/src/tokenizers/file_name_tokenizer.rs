use std::{collections::HashSet, error::Error, path::Path};

use lazy_static::lazy_static;
use regex::Regex;

use crate::tokenizers::tokenizer::Tokenizer;

lazy_static! {
    static ref TOKEN_EXPR: Regex = Regex::new(r"\w+").unwrap();
}

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
            .map(|s| s.to_string())
            .filter(|s| TOKEN_EXPR.is_match(s))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect())
    }
}
