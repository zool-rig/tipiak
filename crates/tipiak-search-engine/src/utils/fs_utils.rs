use std::path::Path;

pub fn is_file_type(path: &Path, extensions: Vec<&str>) -> bool {
    match path.extension() {
        Some(ext) => match ext.to_str() {
            Some(e) => extensions.contains(&e.to_lowercase().as_str()),
            None => false,
        },
        None => false,
    }
}
