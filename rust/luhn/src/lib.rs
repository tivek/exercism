/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if !code.chars().all(|c: char| c.is_ascii_digit() || c == ' ') {
        return false;
    }

    let mut s = 0;
    let mut i = 0;
    for mut n in code.chars().rev().filter_map(|c| c.to_digit(10)) {
        if i % 2 == 1 {
            n *= 2;
            if n > 9 {
                n -= 9
            }
        }
        s += n;
        i += 1;
    }

    i > 1 && s % 10 == 0
}
