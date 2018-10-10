extern crate itertools;

use itertools::Itertools;

pub fn encrypt(input: &str) -> String {
    let mut normalized: String = input.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    normalized.make_ascii_lowercase();

    let mut out = String::new();
    let l = normalized.len();
    let cols = (l as f64).sqrt().ceil() as usize;
    let rows = (l as f64 / cols as f64).ceil() as usize;

    let mut count = 0;
    while count < l {
        for c in 0..cols {
            out.push(normalized[c]);
        }
    }

    out
}
