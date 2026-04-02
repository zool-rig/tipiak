use std::{error::Error, fs, io, path::Path};

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::fs_utils::is_image_file;
use crate::utils::token_utils::{is_indexable_human_text, is_valid_token, sanitize_word};

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

pub struct ExifTokenizer;

impl Tokenizer for ExifTokenizer {
    fn supports(&self, path: &Path) -> bool {
        is_image_file(path)
    }

    fn tokenize(&self, path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
        let mut tokens: Vec<String> = Vec::new();

        let file = fs::File::open(path)?;
        let mut bufreader = io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;

        for f in exif.fields() {
            match f.tag {
                exif::Tag::ImageDescription | exif::Tag::Artist => {
                    let value = &f.display_value().with_unit(&exif).to_string();
                    tokens.extend(
                        value
                            .split_whitespace()
                            .filter(is_valid_token)
                            .map(sanitize_word),
                    );
                }
                exif::Tag::UserComment => {
                    let value = f.display_value().with_unit(&exif).to_string();
                    let bytes = hex_string_to_bytes(&value);
                    if let Some(b) = bytes
                        && let Some(decoded_value) = user_comment_to_string(&b)
                        && is_indexable_human_text(&decoded_value)
                    {
                        tokens.extend(
                            decoded_value
                                .split_whitespace()
                                .filter(is_valid_token)
                                .map(sanitize_word),
                        );
                    }
                }
                _ => (),
            }
        }

        Ok(tokens)
    }
}
