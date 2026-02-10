use std::path::Path;

const MARKDOWN_FILES_EXTENSIONS: [&str; 2] = ["txt", "md"];
const IMAGE_FILES_EXTENSIONS: [&str; 6] = ["tiff", "jpeg", "jpg", "heif", "png", "webp"];

fn is_file_type(path: &Path, extensions: Vec<&str>) -> bool {
    match path.extension() {
        Some(ext) => match ext.to_str() {
            Some(e) => extensions.contains(&e.to_lowercase().as_str()),
            None => false,
        },
        None => false,
    }
}

pub fn is_markdown_file(path: &Path) -> bool {
    is_file_type(path, Vec::from(MARKDOWN_FILES_EXTENSIONS))
}

pub fn is_image_file(path: &Path) -> bool {
    is_file_type(path, Vec::from(IMAGE_FILES_EXTENSIONS))
}

pub fn is_jpeg_file(path: &Path) -> bool {
    is_file_type(path, vec!["jpeg", "jpg"])
}
