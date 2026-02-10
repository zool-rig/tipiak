use crate::utils::token_utils::is_valid_token;

#[derive(Default, Debug)]
pub struct MediaMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub author: Option<String>,
}

impl MediaMetadata {
    pub fn tokenize(&self) -> Vec<String> {
        let mut tokens: Vec<String> = Vec::new();

        if let Some(title) = &self.title {
            tokens.extend(
                title
                .split_whitespace()
                    .filter(is_valid_token)
                    .map(|w| w.to_string()),
            );
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
            tokens.extend(
                tags
                .split_whitespace()
                .filter(is_valid_token)
                .map(|w| w.to_string()),
            );
        }
        
        if let Some(author) = &self.author {
            tokens.extend(
                author
                    .split_whitespace()
                    .filter(is_valid_token)
                    .map(|w| w.to_string()),
            );
        }
        
        tokens
    }

    pub fn is_null(&self) -> bool {
        self.author.is_none()
            && self.description.is_none()
            && self.tags.is_none()
            && self.author.is_none()
    }
}

#[macro_export]
macro_rules! extend_metadata_field {
    ($metadata:ident, $field:ident, $value:expr) => {
        match $metadata.$field {
            Some(v) => $metadata.$field = Some(format!("{} {}", v, $value)),
            None => $metadata.$field = Some($value),
        }
    };
}
