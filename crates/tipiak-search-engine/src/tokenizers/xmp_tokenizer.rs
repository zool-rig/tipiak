use quick_xml::{events::Event, reader::Reader};
use memchr::memmem;
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
    collections::HashSet,
    error::Error
};

use crate::tokenizers::tokenizer::Tokenizer;
use crate::utils::fs_utils::is_file_type;
use crate::utils::token_utils::tokenize_string;

fn extract_xmp_streaming(path: &Path) -> Option<String> {
    let mut file = File::open(path).ok()?;
    
    let file_size = file.metadata().ok()?.len();
    
    // Patterns to search
    let start_finder = memmem::Finder::new(b"<x:xmpmeta");
    let end_finder = memmem::Finder::new(b"</x:xmpmeta>");

    // Scan strategy: read fixed-size blocks without reallocating.
    // 256 KB is a good compromise for CPU cache efficiency.
    const BUFFER_SIZE: usize = 256 * 1024; 
    const OVERLAP: usize = 12; // Length of "</x:xmpmeta>" to handle block boundary splits.
    
    let mut buffer = vec![0u8; BUFFER_SIZE + OVERLAP];
    let mut bytes_in_buffer = 0;
    let mut total_bytes_read: u64 = 0;

    // Mid-file skip threshold for large MP4 files (e.g., 50 MB).
    // If XMP is not near the start, it is often at the very end.
    let early_exit_threshold = 50 * 1024 * 1024; 

    loop {
        // Read data while keeping room for overlap at the beginning.
        let read_offset = bytes_in_buffer;
        let n = file.read(&mut buffer[read_offset..BUFFER_SIZE]).ok()?;
        
        if n == 0 {
            // If we reached the end of the first section with no match and the file is large,
            // jump directly near the end (metadata is often stored there).
            if file_size > early_exit_threshold && total_bytes_read < (file_size - early_exit_threshold) {
                let seek_pos = file_size.saturating_sub(early_exit_threshold);
                if file.seek(SeekFrom::Start(seek_pos)).is_ok() {
                    total_bytes_read = seek_pos;
                    bytes_in_buffer = 0;
                    continue;
                }
            }
            break;
        }

        let current_valid_len = read_offset + n;
        total_bytes_read += n as u64;

        let view = &buffer[..current_valid_len];

        // Very fast SIMD-accelerated search (memchr).
        if let Some(start_pos) = start_finder.find(view) {
            // Found the start marker. Now look for the end marker.
            // It may be in the current buffer, or we keep reading until we find it.
            let mut xmp_data = view[start_pos..].to_vec();
            
            loop {
                if let Some(end_pos) = end_finder.find(&xmp_data) {
                    let final_len = end_pos + 12;
                    xmp_data.truncate(final_len);
                    return String::from_utf8(xmp_data).ok();
                }
                
                // If not found yet, read the next chunk directly into xmp_data.
                let mut chunk = [0u8; 64 * 1024];
                let cn = file.read(&mut chunk).ok()?;
                if cn == 0 { break; }
                xmp_data.extend_from_slice(&chunk[..cn]);
                
                // Guardrail against corrupt files (cap XMP size to 10 MB).
                if xmp_data.len() > 10 * 1024 * 1024 { break; }
            }
            return None;
        }

        // Prepare next block: copy the last 12 bytes to the start of the buffer
        // in case "<x:xmpmeta" was split across two reads.
        if current_valid_len >= OVERLAP {
            buffer.copy_within((current_valid_len - OVERLAP)..current_valid_len, 0);
            bytes_in_buffer = OVERLAP;
        } else {
            bytes_in_buffer = 0;
        }

        // If we scanned the beginning of a large file and found nothing, skip the middle (raw video data).
        if file_size > early_exit_threshold 
            && total_bytes_read > early_exit_threshold 
            && total_bytes_read < (file_size - early_exit_threshold) 
        {
            let seek_pos = file_size.saturating_sub(early_exit_threshold);
            if file.seek(SeekFrom::Start(seek_pos)).is_ok() {
                total_bytes_read = seek_pos;
                bytes_in_buffer = 0;
            }
        }
    }

    None
}

pub struct XmpTokenizer;

impl Tokenizer for XmpTokenizer {
    fn supports(&self, path: &Path) -> bool {
        is_file_type(path, vec!["tiff", "jpeg", "jpg", "heif", "png", "webp", "mp4", "mov"])
    }

    fn tokenize(&self, path: &Path, _root_dir: &Path) -> Result<HashSet<String>, Box<dyn Error>> {
        let mut tokens: HashSet<String> = HashSet::new();

        if let Some(raw_xmp) = extract_xmp_streaming(path) {
            let mut reader = Reader::from_str(&raw_xmp);
            reader.config_mut().trim_text(true);

            let mut buf = Vec::new();
            let mut current_tag: Option<String> = None;
            let mut tag_stack: Vec<String> = Vec::new();

            loop {
                match reader.read_event_into(&mut buf)? {
                    Event::Start(e) => {
                        let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                        tag_stack.push(name.clone());
                        current_tag = Some(name);
                    }
                    Event::Text(e) => {
                        let text = e.xml_content()?.to_string();

                        if text.is_empty() {
                            buf.clear();
                            continue;
                        }

                        match current_tag.as_deref() {
                            Some("photoshop:Headline") => {
                                tokens.extend(tokenize_string(text));
                            }
                            Some("rdf:li") => {
                                if let Some(parent) =
                                    tag_stack.get(tag_stack.len().saturating_sub(3))
                                {
                                    match parent.as_str() {
                                        "dc:title" | "dc:description" | "dc:subject"
                                        | "dc:creator" => {
                                            tokens.extend(tokenize_string(text));
                                        }
                                        _ => (),
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                    Event::Eof => break,
                    _ => (),
                }
                buf.clear();
            }
        }

        Ok(tokens)
    }
}
