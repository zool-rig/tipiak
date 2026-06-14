use quick_xml::{events::Event, reader::Reader};
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
    path::Path,
    collections::HashSet,
};

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::fs_utils::is_image_file;
use crate::utils::token_utils::tokenize_string;

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

pub struct XmpTokenizer;

impl Tokenizer for XmpTokenizer {
    fn supports(&self, path: &Path) -> bool {
        is_image_file(path)
    }

    fn tokenize(&self, path: &Path) -> Result<HashSet<String>, Box<dyn Error>> {
        let mut tokens: HashSet<String> = HashSet::new();

        if let Some(raw_xmp) = extract_xmp_streaming(path) {
            let mut reader = Reader::from_str(&raw_xmp);
            reader.config_mut().trim_text(true);

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
                                tokens.extend(
                                    tokenize_string(text)
                                );
                            }
                            Some("rdf:li") => {
                                if let Some(parent) =
                                    tag_stack.get(tag_stack.len().saturating_sub(3))
                                {
                                    match parent.as_str() {
                                        "dc:title" | "dc:description" | "dc:subject"
                                        | "dc:creator" => {
                                            tokens.extend(
                                                tokenize_string(text)
                                            );
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
        }

        Ok(tokens)
    }
}
