const WORD_CHAR_LIMIT: usize = 3;

pub fn is_valid_token(word: &&str) -> bool {
    word.len() >= WORD_CHAR_LIMIT && !word.replace(|c: char| !c.is_alphanumeric(), "").is_empty()
}
