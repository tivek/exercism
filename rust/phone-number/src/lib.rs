pub fn number(user_number: &str) -> Option<String> {
    let mut out = String::new();
    let mut it = user_number.chars().filter(|c| c.is_ascii_digit());

    let mut one_skipped = false;
    let mut count: usize = 0;

    for c in it.by_ref() {
        if c == '0' || c == '1' {
            if count == 0 {
                if one_skipped {
                    return None;
                } else {
                    one_skipped = true;
                    continue;
                }
            } else if count == 3 {
                return None;
            }
        }
        out.push(c);
        count += 1;
    }

    if out.len() != 10 {
        None
    } else {
        Some(out)
    }
}
