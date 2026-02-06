use std::{error::Error, path::Path};

pub trait Tokenizer: Send + Sync {
    fn supports(&self, path: &Path) -> bool;
    fn tokenize(&self, path: &Path) -> Result<Vec<String>, Box<dyn Error>>;
}
