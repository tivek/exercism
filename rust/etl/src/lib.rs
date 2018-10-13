use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut out = BTreeMap::new();
    for (&score, chars) in h {
        for c in chars {
            out.insert(c.to_ascii_lowercase(), score);
        }
    }

    out
}
