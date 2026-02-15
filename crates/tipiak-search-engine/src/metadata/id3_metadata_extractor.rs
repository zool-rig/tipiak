use id3::{Tag, TagLike};
use std::{error::Error, path::Path};

use crate::metadata::extractor::MetadataExtractor;
use crate::metadata::media_metadata::MediaMetadata;
use crate::utils::fs_utils::is_mp3_file;

pub struct Id3MetadataExtractor;

impl MetadataExtractor for Id3MetadataExtractor {
    fn supports(&self, path: &Path) -> bool {
        is_mp3_file(path)
    }

    fn extract(&self, path: &Path) -> Result<Option<MediaMetadata>, Box<dyn Error>> {
        let tags = Tag::read_from_path(path)?;

        let mut metadata = MediaMetadata::default();

        if let Some(artist) = tags.artist() {
            metadata.author = Some(artist.to_string());
        }

        if let Some(title) = tags.title() {
            metadata.title = Some(title.to_string())
        }
        if let Some(album) = tags.album() {
            metadata.description = Some(album.to_string());
        }

        if let Some(genre) = tags.genre() {
            metadata.tags = Some(genre.to_string())
        }

        Ok(if !metadata.is_null() {
            Some(metadata)
        } else {
            None
        })
    }
}
