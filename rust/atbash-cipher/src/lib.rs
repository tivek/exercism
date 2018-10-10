use std::collections::HashMap;

fn get_mapping() -> HashMap<char, char> {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let numbers = "0123456789";
    let mut out = alphabet
        .chars()
        .zip(alphabet.chars().rev())
        .collect::<HashMap<char, char>>();
    for n in numbers.chars() {
        out.insert(n, n);
    }
    out
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let encoded = decode(plain);
    let mut out = String::new();
    for (i, c) in encoded.char_indices() {
        if i > 0 && i % 5 == 0 {
            out.push(' ');
        }
        out.push(c)
    }
    out
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mapping = get_mapping();
    cipher
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| mapping.get(&c.to_ascii_lowercase()).unwrap())
        .collect()
}
