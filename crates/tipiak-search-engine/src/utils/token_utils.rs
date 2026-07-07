use regex::Regex;
use std::collections::HashSet;
use std::sync::OnceLock;

static STOP_WORDS: OnceLock<HashSet<String>> = OnceLock::new();
static TOKENIZE_EXPR: OnceLock<Regex> = OnceLock::new();

fn get_stop_words() -> &'static HashSet<String> {
    STOP_WORDS.get_or_init(|| {
        let mut stop_words_set: HashSet<String> = HashSet::new();
        let en_stop_words = stop_words::get(stop_words::LANGUAGE::English);
        let fr_stop_words = stop_words::get(stop_words::LANGUAGE::French);
        stop_words_set.extend(en_stop_words.iter().map(|w| w.to_string()));
        stop_words_set.extend(fr_stop_words.iter().map(|w| w.to_string()));
        stop_words_set
    })
}

fn get_tokenize_regex() -> &'static Regex {
    TOKENIZE_EXPR
        .get_or_init(|| Regex::new(r"[a-zA-Z0-9]+").expect("Failed to compile TOKENIZE_EXPR regex"))
}

pub fn tokenize_string(text: String) -> HashSet<String> {
    let stop_words = get_stop_words();
    let tokenize_regex = get_tokenize_regex();
    tokenize_regex
        .find_iter(&text)
        .map(|m| m.as_str().to_lowercase())
        .filter(|token| {
            !(stop_words.contains(token) || token.chars().all(char::is_numeric) && token.len() == 1)
        })
        .collect()
}
