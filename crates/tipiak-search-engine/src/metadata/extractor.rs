use std::{error::Error, path::Path};

use crate::metadata::media_metadata::MediaMetadata;

pub trait MetadataExtractor {
    fn supports(&self, path: &Path) -> bool;
    fn extract(&self, path: &Path) -> Result<Option<MediaMetadata>, Box<dyn Error>>;
}
