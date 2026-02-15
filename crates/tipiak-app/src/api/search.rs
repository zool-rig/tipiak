use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct PayloadFile {
    path: String,
    type_name: String,
}

#[get("/api/search?pattern&filters")]
async fn search(pattern: String, filters: String) -> Result<Vec<PayloadFile>, ServerFnError> {
    use std::path::Path;
    use tipiak_search_engine;

    match tipiak_search_engine::search(
        &Path::new("/home/zool/rust-projects/tipiak/tests_root/"), // TODO
        &pattern,
        match filters.as_str() {
            "all" => None,
            _ => Some(tipiak_search_engine::FileTypeFilters::from_string(filters)),
        },
    ) {
        Ok(matching_files) => Ok(matching_files
            .iter()
            .map(|f| PayloadFile {
                path: f.path.clone(),
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
