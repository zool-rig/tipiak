use std::{path::Path, error::Error};

use crate::metadata::extractor::MetadataExtractor;
use crate::metadata::media_metadata::MediaMetadata;
use crate::utils::fs_utils::is_image_file;

pub struct ExifMetadataExtractor;

impl MetadataExtractor for ExifMetadataExtractor {
    fn supports(&self, path: &Path) -> bool {
        is_image_file(path)
    }

    fn extract(
        &self,
        path: &Path,
    ) -> Result<Option<MediaMetadata>, Box<dyn Error>> {
        todo!()
    }
}
