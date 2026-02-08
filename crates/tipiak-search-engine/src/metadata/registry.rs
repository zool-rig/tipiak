use std::{error::Error, path::Path};

use crate::metadata::exif_metadata_extractor::ExifMetadataExtractor;
use crate::metadata::extractor::MetadataExtractor;
use crate::metadata::media_metadata::MediaMetadata;

pub struct MetadataExtractorRegistry {
    extractors: Vec<Box<dyn MetadataExtractor>>,
}

impl MetadataExtractorRegistry {
    pub fn new() -> Self {
        Self {
            extractors: vec![Box::new(ExifMetadataExtractor)],
        }
    }

    pub fn extract(&self, path: &Path) -> Result<Vec<MediaMetadata>, Box<dyn Error>> {
        Ok(Self
            .extractors
            .iter()
            .filter(|e| e.supports(path))
            .filter_map(|e| e.extract(path).ok()))
            
    }
}
