use std::{collections::HashSet, error::Error, fs, io, path::Path};

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::fs_utils::is_image_file;
use crate::utils::token_utils::tokenize_string;

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

    fn tokenize(&self, path: &Path, _root_dir: &Path) -> Result<HashSet<String>, Box<dyn Error>> {
        let mut tokens: HashSet<String> = HashSet::new();

        let file = fs::File::open(path)?;
        let mut bufreader = io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;

        for f in exif.fields() {
            match f.tag {
                exif::Tag::ImageDescription | exif::Tag::Artist => {
                    let value = &f.display_value().with_unit(&exif).to_string();
                    tokens.extend(tokenize_string(value.to_owned()));
                }
                exif::Tag::UserComment => {
                    let value = f.display_value().with_unit(&exif).to_string();
                    let bytes = hex_string_to_bytes(&value);
                    if let Some(b) = bytes
                        && let Some(decoded_value) = user_comment_to_string(&b)
                    {
                        tokens.extend(tokenize_string(decoded_value));
                    }
                }
                _ => (),
            }
        }

        Ok(tokens)
    }
}
