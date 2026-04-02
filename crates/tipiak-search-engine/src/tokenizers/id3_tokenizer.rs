use id3::{Tag, TagLike};
use std::{error::Error, path::Path};

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::fs_utils::is_mp3_file;
use crate::utils::token_utils::{is_valid_token, sanitize_word};

pub struct Id3Tokenizer;

impl Tokenizer for Id3Tokenizer {
    fn supports(&self, path: &std::path::Path) -> bool {
        is_mp3_file(path)
    }

    fn tokenize(&self, path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
        let mut tokens: Vec<String> = Vec::new();
        let tags = Tag::read_from_path(path)?;

        if let Some(artist) = tags.artist() {
            tokens.extend(
                artist
                    .split_whitespace()
                    .filter(is_valid_token)
                    .map(sanitize_word),
            );
        }

        if let Some(title) = tags.title() {
            tokens.extend(
                title
                    .split_whitespace()
                    .filter(is_valid_token)
                    .map(sanitize_word),
            );
        }

        if let Some(album) = tags.album() {
            tokens.extend(
                album
                    .split_whitespace()
                    .filter(is_valid_token)
                    .map(sanitize_word),
            );
        }

        if let Some(genre) = tags.genre() {
            tokens.extend(
                genre
                    .split_whitespace()
                    .filter(is_valid_token)
                    .map(sanitize_word),
            );
        }

        Ok(tokens)
    }
}
