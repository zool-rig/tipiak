const WORD_CHAR_LIMIT: usize = 3;

pub fn is_valid_token(word: &&str) -> bool {
    word.len() >= WORD_CHAR_LIMIT && !word.replace(|c: char| !c.is_alphabetic(), "").is_empty()
}

fn alphabetic_ratio(s: &str) -> f32 {
    let alpha = s.chars().filter(|c| c.is_alphabetic()).count();
    alpha as f32 / s.len().max(1) as f32
}

fn is_hex_string(s: &str) -> bool {
    s.len() >= 16 && s.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn is_indexable_human_text(s: &str) -> bool {
    if s.len() < 4 {
        return false;
    }

    if is_hex_string(s) {
        return false;
    }

    if alphabetic_ratio(s) < 0.3 {
        return false;
    }

    true
}

pub fn sanitize_word(word: &str) -> String {
    word.replace(|c: char| !c.is_alphanumeric(), "")
}

pub fn sanitize_words(words: &str) -> String {
    words
        .split_whitespace()
        .map(sanitize_word)
        .collect::<Vec<String>>()
        .join(" ")
}
