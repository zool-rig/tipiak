use std::{error::Error, path::Path};

use crate::metadata::registry::MetadataExtractorRegistry;
use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::fs_utils::{is_audio_file, is_image_file};

pub struct MetadataTokenizer;

impl Tokenizer for MetadataTokenizer {
    fn supports(&self, path: &Path) -> bool {
        is_image_file(path) || is_audio_file(path)
    }

    fn tokenize(&self, path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
        let registry = MetadataExtractorRegistry::new();
        Ok(registry
            .extract(path)?
            .iter()
            .flat_map(|m| m.tokenize())
            .collect())
    }
}
