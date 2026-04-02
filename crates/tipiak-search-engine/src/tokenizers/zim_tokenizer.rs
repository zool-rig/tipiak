use std::{error::Error, path::Path};
// use zim::{Namespace, Target, Zim};

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::fs_utils::is_zim_file;
use crate::utils::token_utils::{is_valid_token, sanitize_word};

pub struct ZimTokenizer;

impl Tokenizer for ZimTokenizer {
    fn supports(&self, path: &Path) -> bool {
        is_zim_file(path)
    }

    fn tokenize(&self, path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
        let mut tokens: Vec<String> = Vec::new();

        // let zim_file = Zim::new(path)?;

        // for article_id in &zim_file.article_list {
        //     let entry = zim_file.get_by_url_index(*article_id)?;
        //     match entry.namespace {
        //         Namespace::Metadata => {
        //             println!("{:?}", entry.url);
        //             // match entry.target {
        //             //     Some(Target::Cluster(cluster_num, blob_num)) => {
        //             //         let cluster = zim_file.get_cluster(cluster_num)?;
        //             //         let blob = cluster.get_blob(blob_num)?;
        //             //         match entry.url {

        //             //         }

        //             //     },
        //             //     _ => ()
        //             // }
        //         }
        //         _ => (),
        //     }
        // }

        Ok(tokens)
    }
}
