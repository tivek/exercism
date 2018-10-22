extern crate crossbeam;

use std::cmp::{max, min};
use std::collections::HashMap;

fn fold_hashmap<K: std::cmp::Eq + std::hash::Hash, V: std::ops::AddAssign + Default>(
    a: HashMap<K, V>,
    b: HashMap<K, V>,
) -> HashMap<K, V> {
    let mut a = a;
    for (key, val) in b {
        *a.entry(key).or_default() += val;
    }
    a
}

fn frequency_single_threaded(input: &[&str]) -> HashMap<char, usize> {
    let mut out = HashMap::new();
    for text in input {
        for c in text.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
            *out.entry(c).or_default() += 1;
        }
    }
    out
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    crossbeam::scope(|scope| {
        let mut handlers = Vec::new();
        handlers.reserve_exact(worker_count);
        let n = input.len() / worker_count;
        let mut r = input.len() % worker_count;
        let mut b = 0;
        let mut e = n + min(1, r);
        for _ in 0..worker_count {
            let data = &input[b..e];
            handlers.push(scope.spawn(move || frequency_single_threaded(data)));
            r = max(1, r) - 1;
            b = e;
            e += n + min(1, r);
        }
        handlers
            .into_iter()
            .map(|h| h.join().unwrap())
            .fold(HashMap::new(), fold_hashmap)
    })
}
