use dioxus::prelude::*;

#[get("/api/completion?pattern")]
pub async fn completion(pattern: String) -> Result<Vec<String>, ServerFnError> {
    if pattern.is_empty() {
        return Ok(vec![]);
    }

    use crate::config::Config;
    use std::path::Path;
    use tipiak_search_engine;

    match Config::new() {
        Ok(config) => match tipiak_search_engine::get_all_tokens(Path::new(&config.storage_dir)) {
            Ok(tokens) => {
                let pattern_lower = pattern.to_lowercase();
                Ok(tokens
                    .iter()
                    .filter(|t| {
                        let token_lower = t.to_lowercase();
                        pattern_lower
                            .split_whitespace()
                            .any(|word| token_lower.contains(word))
                    })
                    .map(|t| t.to_owned())
                    .collect())
            }
            Err(e) => Err(ServerFnError::ServerError {
                message: "Failed to search files".to_string(),
                code: 500,
                details: Some(format!("{:?}", e).into()),
            }),
        },
        Err(e) => Err(ServerFnError::ServerError {
            message: "Failed to load config".to_string(),
            code: 500,
            details: Some(format!("{:?}", e).into()),
        }),
    }
}
