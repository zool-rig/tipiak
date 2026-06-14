use std::{collections::HashSet, error::Error, path::Path};

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::token_utils::tokenize_string;

pub struct FileNameTokenizer;

impl Tokenizer for FileNameTokenizer {
    fn supports(&self, _path: &Path) -> bool {
        true
    }

    fn tokenize(&self, path: &Path) -> Result<HashSet<String>, Box<dyn Error>> {
        Ok(tokenize_string(
            path
                .file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string()
        ))
    }
}
