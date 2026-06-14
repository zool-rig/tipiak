use id3::{Tag, TagLike};
use std::{collections::HashSet, error::Error, path::Path};

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::fs_utils::is_mp3_file;
use crate::utils::token_utils::tokenize_string;

pub struct Id3Tokenizer;

impl Tokenizer for Id3Tokenizer {
    fn supports(&self, path: &std::path::Path) -> bool {
        is_mp3_file(path)
    }

    fn tokenize(&self, path: &Path, _root_dir: &Path) -> Result<HashSet<String>, Box<dyn Error>> {
        let mut tokens: HashSet<String> = HashSet::new();
        let tags = Tag::read_from_path(path)?;

        if let Some(artist) = tags.artist() {
            tokens.extend(tokenize_string(artist.to_owned()));
        }

        if let Some(title) = tags.title() {
            tokens.extend(tokenize_string(title.to_owned()));
        }

        if let Some(album) = tags.album() {
            tokens.extend(tokenize_string(album.to_owned()));
        }

        if let Some(genre) = tags.genre() {
            tokens.extend(tokenize_string(genre.to_owned()));
        }

        Ok(tokens)
    }
}
