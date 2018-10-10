/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut count: [u32; 256] = [0; 256];
    for c in sentence
        .chars()
        .filter(|c| c.is_ascii())
        .map(|c| c.to_ascii_uppercase())
    {
        count[c as usize] += 1;
    }

    ('A' as usize..'Z' as usize)
        .map(|c| count[c])
        .all(|i| i == 1)
}
