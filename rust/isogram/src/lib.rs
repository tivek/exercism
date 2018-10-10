pub fn check(candidate: &str) -> bool {
    let mut candidate: Vec<char> = candidate
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_uppercase())
        .collect();
    candidate.sort_unstable();

    let mut letters = candidate.iter();

    match letters.next() {
        None => true,
        Some(mut last) => {
            for c in letters {
                if c == last {
                    return false;
                } else {
                    last = c;
                }
            }
            true
        }
    }
}
