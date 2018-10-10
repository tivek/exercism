use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut out = HashMap::new();

    for word in words
        .split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_ascii_lowercase())
    {
        out.entry(word).and_modify(|e| *e += 1).or_insert(1);
    }
    out
}
