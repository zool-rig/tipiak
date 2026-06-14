use iptc::{IPTC, IPTCTag};
use std::{collections::HashSet, error::Error, path::Path};

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::fs_utils::is_jpeg_file;
use crate::utils::token_utils::tokenize_string;

pub struct IptcTokenizer;

impl Tokenizer for IptcTokenizer {
    fn supports(&self, path: &Path) -> bool {
        is_jpeg_file(path)
    }

    fn tokenize(&self, path: &Path, _root_dir: &Path) -> Result<HashSet<String>, Box<dyn Error>> {
        let mut tokens: HashSet<String> = HashSet::new();
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
                    tokens.extend(value.into_iter().map(tokenize_string).flatten());
                }
                _ => (),
            }
        }

        Ok(tokens)
    }
}
