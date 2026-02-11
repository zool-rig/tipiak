use quick_xml::{events::Event, reader::Reader};
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::extend_metadata_field;
use crate::metadata::extractor::MetadataExtractor;
use crate::metadata::media_metadata::MediaMetadata;
use crate::utils::fs_utils::is_image_file;

fn extract_xmp_streaming(path: &Path) -> Option<String> {
    let file = File::open(path).ok()?;
    let mut reader = BufReader::new(file);

    let mut buffer = [0u8; 8192];
    let mut window = Vec::new();

    loop {
        let n = reader.read(&mut buffer).ok()?;
        if n == 0 {
            break;
        }

        window.extend_from_slice(&buffer[..n]);

        if let Some(start) = window.windows(10).position(|w| w == b"<x:xmpmeta")
            && let Some(end) = window.windows(12).position(|w| w == b"</x:xmpmeta>")
        {
            let xml = &window[start..end + 12];
            return std::str::from_utf8(xml).ok().map(|s| s.to_string());
        }

        // garder une fenêtre glissante raisonnable
        if window.len() > 128 * 1024 {
            window.drain(..window.len() - 128 * 1024);
        }
    }

    None
}

pub struct XmpMetadataExtractor;

impl MetadataExtractor for XmpMetadataExtractor {
    fn supports(&self, path: &Path) -> bool {
        is_image_file(path)
    }

    fn extract(&self, path: &Path) -> Result<Option<MediaMetadata>, Box<dyn Error>> {
        if let Some(raw_xmp) = extract_xmp_streaming(path) {
            let mut reader = Reader::from_str(&raw_xmp);
            reader.config_mut().trim_text(true);

            let mut metadata = MediaMetadata::default();
            let mut buf = Vec::new();
            let mut current_tag: Option<String> = None;
            let mut tag_stack: Vec<String> = Vec::new();

            loop {
                match reader.read_event_into(&mut buf)? {
                    Event::Start(e) => {
                        let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                        tag_stack.push(name.clone());
                        current_tag = Some(name);
                    }
                    Event::Text(e) => {
                        let text = e.xml_content()?.to_string();

                        if text.is_empty() {
                            buf.clear();
                            continue;
                        }

                        match current_tag.as_deref() {
                            Some("photoshop:Headline") => {
                                extend_metadata_field!(metadata, title, text);
                            }
                            Some("rdf:li") => {
                                if let Some(parent) =
                                    tag_stack.get(tag_stack.len().saturating_sub(3))
                                {
                                    match parent.as_str() {
                                        "dc:title" => {
                                            extend_metadata_field!(metadata, title, text);
                                        }
                                        "dc:description" => {
                                            extend_metadata_field!(metadata, description, text);
                                        }
                                        "dc:subject" => {
                                            extend_metadata_field!(metadata, tags, text);
                                        }
                                        "dc:creator" => {
                                            extend_metadata_field!(metadata, author, text);
                                        }
                                        _ => (),
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                    Event::Eof => break,
                    _ => (),
                }
                buf.clear();
            }

            return Ok(if !metadata.is_null() {
                Some(metadata)
            } else {
                None
            });
        }

        Ok(None)
    }
}
