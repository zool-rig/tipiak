use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Deserialize, Serialize, Debug)]
pub struct PayloadFile {
    pub id: i64,
    pub path: String,
    pub type_name: String,
}

#[get("/api/search?pattern&filters")]
pub async fn search(
    pattern: String,
    filters: Vec<bool>,
) -> Result<Vec<PayloadFile>, ServerFnError> {
    use std::path::Path;
    use tipiak_search_engine;

    let se_config = &tipiak_search_engine::CONFIG;
    let filters_str = se_config
        .file_types
        .iter()
        .enumerate()
        .filter(|x| filters[x.0])
        .map(|x| x.1 .0.to_string())
        .collect::<Vec<String>>()
        .join("|");

    match Config::new() {
        Ok(config) => {
            match tipiak_search_engine::search(
                &Path::new(&config.storage_dir),
                &pattern,
                Some(tipiak_search_engine::FileTypeFilters::from_string(
                    filters_str,
                )),
            ) {
                Ok(matching_files) => Ok(matching_files
                    .iter()
                    .map(|f| PayloadFile {
                        id: f.id,
                        path: Path::new(&f.path)
                            .strip_prefix(&config.storage_dir)
                            .unwrap_or(Path::new(&f.path))
                            .to_string_lossy()
                            .into_owned(),
                        type_name: f.type_name.clone(),
                    })
                    .collect()),
                Err(e) => Err(ServerFnError::ServerError {
                    message: "Failed to search files".to_string(),
                    code: 500,
                    details: Some(format!("{:?}", e).into()),
                }),
            }
        }
        Err(e) => Err(ServerFnError::ServerError {
            message: "Failed to load config".to_string(),
            code: 500,
            details: Some(format!("{:?}", e).into()),
        }),
    }
}
