mod config;
mod constants;
mod crawler;
mod db;
mod models;
mod searcher;
mod tokenizers;
mod utils;

pub use crate::config::get_config;
pub use crate::crawler::crawl;
pub use crate::models::file::File;
pub use crate::models::file_type::FileType;
pub use crate::searcher::{FileTypeFilters, search};
pub use crate::utils::db_utils::{get_all_tokens, get_path_from_id};

#[cfg(test)]
mod tests {

    #[test]
    fn tokenize_string() {
        use crate::utils::token_utils::tokenize_string;
        use std::collections::HashSet;
        let tokens = tokenize_string("toto abc_defg-xyz 2026!:dsq;da".into());
        assert_eq!(
            tokens,
            vec!["2026", "abc", "xyz", "defg", "dsq", "toto"]
                .into_iter()
                .map(|t| t.to_string())
                .collect::<HashSet<String>>()
        )
    }
}
