fn build_chain(chain: Vec<(u8, u8)>, input: Vec<(u8, u8)>) -> Vec<Vec<(u8, u8)>> {
    if input.is_empty() {
        return vec![chain];
    }
    let anchor = chain.last().unwrap().1;
    let mut out = Vec::new();
    for i in 0..input.len() {
        let mut remaining = input.clone();
        let mut d = remaining.remove(i);
        if d.1 == anchor {
            d = (d.1, d.0);
        } else if d.0 != anchor {
            continue;
        }
        let mut new_chain = chain.clone();
        new_chain.push(d);
        for built_chain in build_chain(new_chain, remaining) {
            out.push(built_chain);
        }
    }
    out
}

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        return Some(Vec::new());
    }
    for c in build_chain(input[0..1].to_vec(), input[1..].to_vec()) {
        if c.first().unwrap().0 == c.last().unwrap().1 {
            return Some(c);
        }
    }
    None
}
