use std::{collections::HashSet, error::Error, path::Path};

use crate::tokenizers::{
    file_name_tokenizer::FileNameTokenizer, paragraph_tokenizer::ParagraphTokenizer,
    title_tokenizer::MarkdownTitleTokenizer, tokenizer::Tokenizer,
    metadata_tokenizer::MetadataTokenizer,
};

pub struct TokenizerRegistry {
    tokenizers: Vec<Box<dyn Tokenizer>>,
}

impl TokenizerRegistry {
    pub fn new() -> Self {
        Self {
            tokenizers: vec![
                Box::new(FileNameTokenizer),
                Box::new(ParagraphTokenizer),
                Box::new(MarkdownTitleTokenizer),
                Box::new(MetadataTokenizer),
            ],
        }
    }

    pub fn tokenize(&self, path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(self
            .tokenizers
            .iter()
            .filter(|t| t.supports(path))
            .filter_map(|t| t.tokenize(path).ok())
            .flatten()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect())
    }
}
