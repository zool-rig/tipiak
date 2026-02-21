use dioxus::{fullstack::FileStream, prelude::*};

use crate::config::Config;

#[get("/api/media/:id")]
pub async fn media(id: i64) -> Result<FileStream, ServerFnError> {
    use std::path::Path;
    use tipiak_search_engine;

    match Config::new() {
        Ok(config) => {
            match tipiak_search_engine::get_path_from_id(&Path::new(&config.storage_dir), id) {
                Ok(Some(path)) => match FileStream::from_path(&path).await {
                    Ok(stream) => Ok(stream),
                    Err(e) => Err(ServerFnError::ServerError {
                        message: format!("Failed to stream file : {:?}", path),
                        code: 500,
                        details: Some(format!("{:?}", e).into()),
                    }),
                },
                Ok(None) => Err(ServerFnError::ServerError {
                    message: "File not found".to_string(),
                    code: 404,
                    details: Some(format!("id : {}", id).into()),
                }),
                Err(e) => Err(ServerFnError::ServerError {
                    message: "Failed get path from id".to_string(),
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
