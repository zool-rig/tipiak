use crate::utils::token_utils::is_valid_token;

pub struct MediaMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub author: Option<String>,
}

impl MediaMetadata {
    pub fn tokenize(&self) -> Vec<String> {
        let mut tokens: Vec<String> = Vec::new();

        if let Some(title) = &self.title {
            tokens.push(title.clone())
        }

        if let Some(description) = &self.description {
            tokens.extend(
                description
                    .split_whitespace()
                    .filter(is_valid_token)
                    .map(|w| w.to_string()),
            );
        }

        if let Some(tags) = &self.tags {
            tokens.extend(tags.clone())
        }

        if let Some(author) = &self.author {
            tokens.push(author.clone())
        }

        tokens
    }
}
