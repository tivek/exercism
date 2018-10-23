use std::collections::HashSet;

fn sorted_letters(word: &str) -> Vec<char> {
    let mut out: Vec<char> = word.to_lowercase().chars().collect();
    out.sort_unstable();
    out
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lcase = word.to_lowercase();
    let sorted_word = sorted_letters(word);
    possible_anagrams
        .iter()
        .filter(|pa| sorted_letters(pa) == sorted_word && pa.to_lowercase() != lcase)
        .cloned()
        .collect()
}
