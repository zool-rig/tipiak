use iptc::{IPTC, IPTCTag};
use std::{error::Error, path::Path};

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::fs_utils::is_jpeg_file;
use crate::utils::token_utils::{is_valid_token, sanitize_word};

pub struct IptcTokenizer;

impl Tokenizer for IptcTokenizer {
    fn supports(&self, path: &Path) -> bool {
        is_jpeg_file(path)
    }

    fn tokenize(&self, path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
        let mut tokens: Vec<String> = Vec::new();
        let iptc = IPTC::read_from_path(path)?;

        for (tag, value) in iptc.get_all() {
            match tag {
                IPTCTag::ObjectName
                | IPTCTag::Headline
                | IPTCTag::Caption
                | IPTCTag::LocalCaption
                | IPTCTag::RasterizedCaption
                | IPTCTag::Keywords
                | IPTCTag::SupplementalCategories
                | IPTCTag::Category
                | IPTCTag::SubjectReference
                | IPTCTag::ByLine
                | IPTCTag::ByLineTitle
                | IPTCTag::Credit => {
                    tokens.extend(
                        value
                            .iter()
                            .filter(|w| is_valid_token(&w.as_str()))
                            .map(|w| sanitize_word(w)),
                    );
                }
                _ => (),
            }
        }

        Ok(tokens)
    }
}
