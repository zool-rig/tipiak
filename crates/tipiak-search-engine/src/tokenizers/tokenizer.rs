use std::{collections::HashSet, error::Error, path::Path};

pub trait Tokenizer: Send + Sync {
    fn supports(&self, path: &Path) -> bool;
    fn tokenize(&self, path: &Path, root_dir: &Path) -> Result<HashSet<String>, Box<dyn Error>>;
}
