use std::collections::HashSet;
use regex::Regex;
use std::sync::OnceLock;

static STOP_WORDS: OnceLock<HashSet<String>> = OnceLock::new();
static TOKENIZE_EXPR: OnceLock<Regex> = OnceLock::new();

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

fn get_stop_words() -> &'static HashSet<String> {
    STOP_WORDS.get_or_init(|| {
        let mut stop_words_set: HashSet<String> = HashSet::new();
        let en_stop_words = stop_words::get(stop_words::LANGUAGE::English);
        let fr_stop_words = stop_words::get(stop_words::LANGUAGE::French);
        stop_words_set.extend(en_stop_words.into_iter().map(|w| w.to_string()));
        stop_words_set.extend(fr_stop_words.into_iter().map(|w| w.to_string()));
        stop_words_set
    })
}

fn get_tokenize_regex() -> &'static Regex {
    TOKENIZE_EXPR.get_or_init(|| {
        Regex::new(r"/[a-zA-Z0-9]+/gm").expect("Failed to compile TOKENIZE_EXPR regex")
    })
}

pub fn tokenize_string(text: String) -> HashSet<String> {
    let stop_words = get_stop_words();
    let tokenize_regex = get_tokenize_regex();
    tokenize_regex.find_iter(&text)
        .map(|m| m.as_str().to_lowercase())
        .filter(|token| {
            !stop_words.contains(token) && !(token.chars().all(char::is_numeric) && token.len() == 1)
        })
        .collect()
}