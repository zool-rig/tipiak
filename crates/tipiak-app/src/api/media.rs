use dioxus::{fullstack::FileStream, prelude::*};

#[get("/api/media/:id")]
pub async fn media(id: i64) -> Result<FileStream, ServerFnError> {
    use crate::config::Config;
    use std::path::Path;
    use tipiak_search_engine;

    match Config::new() {
        Ok(config) => {
            match tipiak_search_engine::get_path_from_id(Path::new(&config.storage_dir), id) {
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

#[cfg(feature = "server")]
pub mod download {
    use dioxus::server::axum::{
        body::Body,
        extract::Path,
        http::{header, StatusCode},
        response::{IntoResponse, Response},
    };
    use tokio_util::io::ReaderStream;

    pub async fn media_download(Path(id): Path<i64>) -> Response {
        use crate::config::Config;
        use std::path::Path as StdPath;

        let config = match Config::new() {
            Ok(c) => c,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        let path =
            match tipiak_search_engine::get_path_from_id(StdPath::new(&config.storage_dir), id) {
                Ok(Some(p)) => p,
                Ok(None) => return StatusCode::NOT_FOUND.into_response(),
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            };

        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("download")
            .to_string();

        let content_type = mime_guess::from_path(&path)
            .first_or_octet_stream()
            .to_string();

        let file = match tokio::fs::File::open(&path).await {
            Ok(f) => f,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        let body = Body::from_stream(ReaderStream::new(file));

        (
            [
                (header::CONTENT_TYPE, content_type),
                (
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", filename),
                ),
            ],
            body,
        )
            .into_response()
    }
}
