use std::collections::HashMap;

fn is_nucleotide(nucleotide: char) -> bool {
    match nucleotide {
        'C' | 'T' | 'A' | 'G' => true,
        _ => false,
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !is_nucleotide(nucleotide) {
        return Err(nucleotide);
    }
    let res = nucleotide_counts(dna);
    res.map(|cnts| cnts.get(&nucleotide).cloned().unwrap())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut res = HashMap::with_capacity(4);
    for c in "CTAG".chars() {
        res.insert(c, 0);
    }
    for c in dna.chars() {
        if is_nucleotide(c) {
            let mut cnt = res.get_mut(&c).unwrap();
            *cnt += 1;
        } else {
            return Err(c);
        }
    }
    Ok(res)
}
