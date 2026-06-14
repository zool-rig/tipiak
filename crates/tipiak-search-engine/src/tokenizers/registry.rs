use std::{error::Error, path::{Path, PathBuf}};
use rayon::prelude::*;

use crate::tokenizers::{
    exif_tokenizer::ExifTokenizer, file_path_tokenizer::FilePathTokenizer,
    id3_tokenizer::Id3Tokenizer, iptc_tokenizer::IptcTokenizer,
    paragraph_tokenizer::ParagraphTokenizer, title_tokenizer::MarkdownTitleTokenizer,
    tokenizer::Tokenizer, xmp_tokenizer::XmpTokenizer, zim_tokenizer::ZimTokenizer,
};

pub struct TokenizerRegistry {
    root_dir: PathBuf,
    tokenizers: Vec<Box<dyn Tokenizer>>,
}

impl TokenizerRegistry {
    pub fn new(root_dir: &Path) -> Self {
        Self {
            root_dir: PathBuf::from(root_dir),
            tokenizers: vec![
                Box::new(FilePathTokenizer),
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
            .par_iter()
            .filter(|t| t.supports(path))
            .filter_map(|t| t.tokenize(path, &self.root_dir).ok())
            .flatten()
            .collect())
    }
}
