use std::{collections::HashSet, error::Error, path::Path};

use crate::tokenizers::{
    exif_tokenizer::ExifTokenizer, file_name_tokenizer::FileNameTokenizer,
    id3_tokenizer::Id3Tokenizer, iptc_tokenizer::IptcTokenizer,
    paragraph_tokenizer::ParagraphTokenizer, title_tokenizer::MarkdownTitleTokenizer,
    tokenizer::Tokenizer, xmp_tokenizer::XmpTokenizer, zim_tokenizer::ZimTokenizer,
};
use crate::utils::token_utils::sanitize_word;

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
                Box::new(Id3Tokenizer),
                Box::new(ExifTokenizer),
                Box::new(IptcTokenizer),
                Box::new(XmpTokenizer),
                Box::new(ZimTokenizer),
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
            .map(|t| sanitize_word(&t.to_lowercase()))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect())
    }
}
