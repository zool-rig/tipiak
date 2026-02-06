use std::{error::Error, path::Path};

use crate::tokenizers::{file_name_tokenizer::FileNameTokenizer, tokenizer::Tokenizer};

pub struct TokenizerRegistry {
    tokenizers: Vec<Box<dyn Tokenizer>>,
}

impl TokenizerRegistry {
    pub fn new() -> Self {
        Self {
            tokenizers: vec![Box::new(FileNameTokenizer)],
        }
    }

    pub fn tokenize(&self, path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(self
            .tokenizers
            .iter()
            .filter(|t| t.supports(path))
            .filter_map(|t| t.tokenize(path).ok())
            .flatten()
            .collect())
    }
}
