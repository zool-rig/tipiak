use std::{
    collections::HashSet,
    fs,
    io::{self, BufRead},
    path::Path,
};

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::fs_utils::is_markdown_file;
use crate::utils::token_utils::is_valid_token;

const TOKEN_LIMIT: usize = 10;

pub struct ParagraphTokenizer;

impl Tokenizer for ParagraphTokenizer {
    fn supports(&self, path: &Path) -> bool {
        is_markdown_file(path)
    }

    fn tokenize(&self, path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let file = fs::File::open(path)?;
        let buffer = io::BufReader::new(file);

        let mut tokens: HashSet<String> = HashSet::new();

        for line in buffer.lines().map_while(Result::ok) {
            for word in line.split_whitespace() {
                if is_valid_token(&word) {
                    tokens.insert(word.to_string());
                }
                if tokens.len() == TOKEN_LIMIT {
                    break;
                }
            }
        }
        Ok(tokens.into_iter().collect())
    }
}
