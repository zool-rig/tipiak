use std::{path::Path, error::Error};

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::fs_utils::is_zim_file;

pub struct ZimTokenizer;

impl Tokenizer for ZimTokenizer {
    fn supports(&self, path: &Path) -> bool {
        is_zim_file(path)
    }

    fn tokenize(&self, path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(vec![])
    }
}