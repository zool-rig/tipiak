use iptc::{IPTC, IPTCTag};
use std::{error::Error, path::Path};

use crate::metadata::extractor::MetadataExtractor;
use crate::metadata::media_metadata::MediaMetadata;
use crate::utils::fs_utils::is_jpeg_file;
use crate::utils::token_utils::sanitize_words;
use crate::extend_metadata_field;

pub struct IptcMetadataExtractor;

impl MetadataExtractor for IptcMetadataExtractor {
    fn supports(&self, path: &Path) -> bool {
        is_jpeg_file(path)
    }

    fn extract(&self, path: &Path) -> Result<Option<MediaMetadata>, Box<dyn Error>> {
        let iptc = IPTC::read_from_path(path)?;
        let mut metadata = MediaMetadata::default();

        for (tag, value) in iptc.get_all() {
            match tag {
                IPTCTag::ObjectName => extend_metadata_field!(
                    metadata,
                    title,
                    sanitize_words(&value.join(" "))
                ),
                IPTCTag::Headline => extend_metadata_field!(
                    metadata,
                    title,
                    sanitize_words(&value.join(" "))
                ),
                IPTCTag::Caption => extend_metadata_field!(
                    metadata,
                    description,
                    sanitize_words(&value.join(" "))
                ),
                IPTCTag::LocalCaption => extend_metadata_field!(
                    metadata,
                    description,
                    sanitize_words(&value.join(" "))
                ),
                IPTCTag::RasterizedCaption => extend_metadata_field!(
                    metadata,
                    description,
                    sanitize_words(&value.join(" "))
                ),
                IPTCTag::Keywords => extend_metadata_field!(
                    metadata,
                    tags,
                    sanitize_words(&value.join(" "))
                ),
                IPTCTag::SupplementalCategories => extend_metadata_field!(
                    metadata,
                    tags,
                    sanitize_words(&value.join(" "))
                ),
                IPTCTag::Category => extend_metadata_field!(
                    metadata,
                    tags,
                    sanitize_words(&value.join(" "))
                ),
                IPTCTag::SubjectReference => extend_metadata_field!(
                    metadata,
                    tags,
                    sanitize_words(&value.join(" "))
                ),
                IPTCTag::ByLine => extend_metadata_field!(
                    metadata,
                    author,
                    sanitize_words(&value.join(" "))
                ),
                IPTCTag::ByLineTitle => extend_metadata_field!(
                    metadata,
                    author,
                    sanitize_words(&value.join(" "))
                ),
                IPTCTag::Credit => extend_metadata_field!(
                    metadata,
                    author,
                    sanitize_words(&value.join(" "))
                ),
                _ => (),
            }
        }

        Ok(if !metadata.is_null() {
            Some(metadata)
        } else {
            None
        })
    }
}
