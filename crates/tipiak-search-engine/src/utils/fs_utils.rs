use std::path::Path;

const MARKDOWN_FILES_EXTENSIONS: [&str; 2] = ["txt", "md"];
const IMAGE_FILES_EXTENSIONS: [&str; 6] = ["tiff", "jpeg", "jpg", "heif", "png", "webp"];
const AUDIO_FILES_EXTENSIONS: [&str; 2] = ["mp3", "wav"];
const JPEG_FILES_EXTENSIONS: [&str; 2] = ["jpeg", "jpg"];

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
    is_file_type(path, Vec::from(JPEG_FILES_EXTENSIONS))
}

pub fn is_audio_file(path: &Path) -> bool {
    is_file_type(path, Vec::from(AUDIO_FILES_EXTENSIONS))
}

pub fn is_mp3_file(path: &Path) -> bool {
    is_file_type(path, vec!["mp3"])
}

pub fn is_zim_file(path: &Path) -> bool {
    is_file_type(path, vec!["zim"])
}