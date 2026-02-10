use std::{error::Error, path::Path};

use crate::metadata::exif_metadata_extractor::ExifMetadataExtractor;
use crate::metadata::extractor::MetadataExtractor;
use crate::metadata::iptc_metadata_extractor::IptcMetadataExtractor;
use crate::metadata::media_metadata::MediaMetadata;

pub struct MetadataExtractorRegistry {
    extractors: Vec<Box<dyn MetadataExtractor>>,
}

impl MetadataExtractorRegistry {
    pub fn new() -> Self {
        Self {
            extractors: vec![
                Box::new(ExifMetadataExtractor),
                Box::new(IptcMetadataExtractor),
            ],
        }
    }

    pub fn extract(&self, path: &Path) -> Result<Vec<MediaMetadata>, Box<dyn Error>> {
        Ok(self
            .extractors
            .iter()
            .filter(|e| e.supports(path))
            .filter_map(|e| Some(e.extract(path).unwrap_or_else(|e| {
                println!("{:?} {:?}", path, e);
                None
            })))
            .flatten()
            .collect())
    }
}
