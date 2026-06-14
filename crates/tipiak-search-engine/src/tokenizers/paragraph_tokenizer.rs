use std::{
    collections::HashSet,
    error::Error,
    fs,
    io::{self, BufRead},
    path::Path,
};

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::fs_utils::is_markdown_file;
use crate::utils::token_utils::tokenize_string;

const TOKEN_LIMIT: usize = 10;

pub struct ParagraphTokenizer;

impl Tokenizer for ParagraphTokenizer {
    fn supports(&self, path: &Path) -> bool {
        is_markdown_file(path)
    }

    fn tokenize(&self, path: &Path, _root_dir: &Path) -> Result<HashSet<String>, Box<dyn Error>> {
        let file = fs::File::open(path)?;
        let buffer = io::BufReader::new(file);

        let mut tokens: HashSet<String> = HashSet::new();

        for line in buffer.lines().map_while(Result::ok) {
            if tokens.len() >= TOKEN_LIMIT {
                break;
            }
            tokens.extend(tokenize_string(line));
        }
        Ok(tokens.into_iter().collect())
    }
}
