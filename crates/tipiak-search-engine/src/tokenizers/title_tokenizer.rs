use regex::Regex;
use std::io::BufRead;
use std::{collections::HashSet, fs, io, path::Path, sync::OnceLock};

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::fs_utils::is_markdown_file;
use crate::utils::token_utils::tokenize_string;

static MD_TITLE_EXPR: OnceLock<Regex> = OnceLock::new();

fn get_md_title_expr() -> &'static Regex {
    MD_TITLE_EXPR.get_or_init(|| Regex::new(r"#{1,3}\s*(\w+)").expect("Failed to compile regex"))
}

pub struct MarkdownTitleTokenizer;

impl Tokenizer for MarkdownTitleTokenizer {
    fn supports(&self, path: &Path) -> bool {
        is_markdown_file(path)
    }

    fn tokenize(&self, path: &Path, _root_dir: &Path) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
        let file = fs::File::open(path)?;
        let buffer = io::BufReader::new(file);
        let md_title_expr = get_md_title_expr();
        Ok(buffer
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| {
                md_title_expr
                    .captures(&line)
                    .and_then(|caps| caps.get(1))
                    .map(|m| tokenize_string(m.as_str().to_string()))
            })
            .flatten()
            .collect())
    }
}
