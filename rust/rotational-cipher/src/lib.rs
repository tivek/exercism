pub fn rotate(input: &str, key: i8) -> String {
    let mut out = String::with_capacity(input.len());
    for c in input.chars() {
        if !c.is_ascii_alphabetic() {
            out.push(c)
        } else {
            let base = {
                if c.is_uppercase() {
                    'A' as i32
                } else {
                    'a' as i32
                }
            };
            let diff = 'Z' as i32 - 'A' as i32 + 1;
            out.push(((c as i32 - base + key as i32) % diff + base) as u8 as char)
        }
    }
    out
}
