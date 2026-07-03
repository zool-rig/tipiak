use dioxus::prelude::*;

#[get("/api/config/file-types")]
pub async fn file_types() -> Result<Vec<String>> {
    use tipiak_search_engine;
    let mut file_types_names: Vec<String> = tipiak_search_engine::get_config()
        .file_types
        .iter()
        .map(|x| x.0.clone())
        .collect();
    file_types_names.sort();
    Ok(file_types_names)
}
