use std::collections::HashMap;

#[derive(Default, Clone, Copy, PartialEq, Debug)]
struct LetterEntry {
    pub non_zero: bool,
    pub factor: i32,
}

impl LetterEntry {
    pub fn new() -> LetterEntry {
        LetterEntry {
            non_zero: false,
            factor: 0,
        }
    }
}

fn parse_puzzle(input: &str) -> Option<(Vec<char>, Vec<LetterEntry>)> {
    let mut factors: HashMap<char, LetterEntry> = HashMap::new();

    let mut found_eq = false;
    for token in input.split_whitespace() {
        if token == "==" {
            if found_eq {
                return None; // can't have "==" more than once
            } else {
                found_eq = true;
            }
            continue;
        }

        if token == "+" {
            continue;
        }

        if !token.chars().all(|c| c.is_ascii_alphabetic()) {
            return None;
        }
        let mut cur_factor = 1;
        for letter in token.chars().rev() {
            let entry = factors.entry(letter).or_insert(LetterEntry::new());
            if found_eq {
                entry.factor -= cur_factor;
            } else {
                entry.factor += cur_factor;
            }
            cur_factor *= 10;
        }
        if let Some(first_letter) = token.chars().next() {
            factors.get_mut(&first_letter).unwrap().non_zero = true;
        } else {
            return None;
        }
    }

    if factors.len() > 10 {
        None
    } else {
        Some((
            factors.keys().cloned().collect(),
            factors.values().cloned().collect(),
        ))
    }
}

fn choose_digits(
    factors: &[LetterEntry],
    target_sum: i32,
    available_digits: Vec<u8>,
) -> Option<Vec<u8>> {
    if factors.len() == 0 {
        if target_sum == 0 {
            return Some(Vec::new());
        } else {
            return None;
        }
    }
    for idx in 0..available_digits.len() {
        let mut new_available_digits = available_digits.clone();
        let d = new_available_digits.remove(idx);
        if factors[0].non_zero && d == 0 {
            continue;
        }
        let new_target_sum = target_sum - d as i32 * &factors[0].factor;
        match choose_digits(&factors[1..], new_target_sum, new_available_digits) {
            None => continue,
            Some(mut v) => {
                v.insert(0, d);
                return Some(v);
            }
        }
    }
    None
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    if let Some((letters, factors)) = parse_puzzle(input) {
        if let Some(digits) = choose_digits(&factors[..], 0, (0..10 as u8).collect()) {
            return Some(letters.into_iter().zip(digits).collect());
        }
    }
    None
}
