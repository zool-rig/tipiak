use std::{error::Error, path::Path, collections::HashSet};

pub trait Tokenizer {
    fn supports(&self, path: &Path) -> bool;
    fn tokenize(&self, path: &Path) -> Result<HashSet<String>, Box<dyn Error>>;
}
