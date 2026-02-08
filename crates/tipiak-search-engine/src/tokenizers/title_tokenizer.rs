use lazy_static::lazy_static;
use regex::Regex;
use std::io::BufRead;
use std::{fs, io, path::Path};

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::fs_utils::is_markdown_file;

lazy_static! {
    static ref MD_TITLE_EXPR: Regex =
        Regex::new(r"#{1,3}\s*(\w+)").expect("Failed to compile regex");
}

pub struct MarkdownTitleTokenizer;

impl Tokenizer for MarkdownTitleTokenizer {
    fn supports(&self, path: &Path) -> bool {
        is_markdown_file(path)
    }

    fn tokenize(&self, path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let file = fs::File::open(path)?;
        let buffer = io::BufReader::new(file);
        Ok(buffer
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| {
                MD_TITLE_EXPR
                    .captures(&line)
                    .and_then(|caps| caps.get(1))
                    .map(|m| m.as_str().to_string())
            })
            .collect())
    }
}
