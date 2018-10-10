pub fn abbreviate(phrase: &str) -> String {
    let mut out = String::new();
    for word in phrase
        .split(|c: char| c.is_whitespace() || c == '-')
        .filter(|s| !s.is_empty())
    {
        println!("{:?}", word);
        if word
            .chars()
            .all(|c| c.is_ascii_uppercase() || !c.is_ascii_alphabetic())
        {
            for c in word.chars() {
                if c.is_ascii_alphabetic() {
                    out.push(c.to_ascii_uppercase());
                    break;
                }
            }
        } else {
            for (i, c) in word.chars().enumerate() {
                if i == 0 || c.is_ascii_uppercase() {
                    out.push(c.to_ascii_uppercase());
                }
            }
        }
    }
    out
}
