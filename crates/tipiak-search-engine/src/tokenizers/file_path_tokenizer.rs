use std::{collections::HashSet, error::Error, path::Path};

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::token_utils::tokenize_string;

pub struct FilePathTokenizer;

impl Tokenizer for FilePathTokenizer {
    fn supports(&self, _path: &Path) -> bool {
        true
    }

    fn tokenize(&self, path: &Path, root_dir: &Path) -> Result<HashSet<String>, Box<dyn Error>> {
        Ok(tokenize_string(path
            .strip_prefix(root_dir)
            .unwrap_or(&Path::new(""))
            .to_str()
            .unwrap_or_default()
            .to_string()
        ))
    }
}
