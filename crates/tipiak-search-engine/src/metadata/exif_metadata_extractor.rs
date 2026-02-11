use std::{error::Error, fs, io, path::Path};

use crate::metadata::extractor::MetadataExtractor;
use crate::metadata::media_metadata::MediaMetadata;
use crate::utils::fs_utils::is_image_file;
use crate::utils::token_utils::{is_indexable_human_text, sanitize_words};

fn hex_string_to_bytes(s: &str) -> Option<Vec<u8>> {
    let s = s.strip_prefix("0x")?;

    if s.len() % 2 != 0 {
        return None;
    }

    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok())
        .collect()
}

fn user_comment_to_string(raw: &[u8]) -> Option<String> {
    if raw.len() < 8 {
        return None;
    }

    let (encoding, data) = raw.split_at(8);

    let decoded = match encoding {
        b"ASCII\0\0\0" => std::str::from_utf8(data).ok()?,
        b"UNICODE\0" => return None, // Not supported yet
        _ => return None,
    };

    let cleaned = decoded.trim_matches(|c: char| c.is_whitespace() || c == '\u{0}');

    if cleaned.len() >= 3 {
        Some(cleaned.to_string())
    } else {
        None
    }
}

pub struct ExifMetadataExtractor;

impl MetadataExtractor for ExifMetadataExtractor {
    fn supports(&self, path: &Path) -> bool {
        is_image_file(path)
    }

    fn extract(&self, path: &Path) -> Result<Option<MediaMetadata>, Box<dyn Error>> {
        let file = fs::File::open(path)?;
        let mut bufreader = io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;

        let mut metadata = MediaMetadata::default();

        for f in exif.fields() {
            match f.tag {
                exif::Tag::ImageDescription => {
                    let value = sanitize_words(&f.display_value().with_unit(&exif).to_string());
                    if !value.is_empty() {
                        metadata.title = Some(value);
                    }
                }
                exif::Tag::Artist => {
                    let value = sanitize_words(&f.display_value().with_unit(&exif).to_string());
                    if !value.is_empty() {
                        metadata.author = Some(value);
                    }
                }
                exif::Tag::UserComment => {
                    let value = f.display_value().with_unit(&exif).to_string();
                    let bytes = hex_string_to_bytes(&value);
                    if let Some(b) = bytes
                        && let Some(decoded_value) = user_comment_to_string(&b)
                        && is_indexable_human_text(&decoded_value)
                    {
                        let sanitized_value = sanitize_words(&decoded_value);
                        if !sanitized_value.is_empty() {
                            metadata.description = Some(sanitized_value);
                        }
                    }
                }
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
