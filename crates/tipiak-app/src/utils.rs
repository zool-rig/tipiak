pub fn encode_filters(filters: &[bool]) -> String {
    filters.iter().map(|b| if *b { '1' } else { '0' }).collect()
}

pub fn decode_filters(s: &str) -> Vec<bool> {
    s.chars().map(|c| c == '1').collect()
}
