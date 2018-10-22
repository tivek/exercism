extern crate crossbeam;

use std::collections::HashMap;
use std::sync::mpsc;

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

fn worker(rx: mpsc::Receiver<&str>) -> HashMap<char, usize> {
    let mut out = HashMap::new();
    for text in rx {
        for c in text.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
            *out.entry(c).or_default() += 1;
        }
    }
    out
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    crossbeam::scope(|scope| {
        {
            let mut handlers = Vec::new();
            let mut senders = Vec::new();

            for _ in 0..worker_count {
                let (tx, rx) = mpsc::channel();
                handlers.push(scope.spawn(move || worker(rx)));
                senders.push(tx);
            }

            for (text, tx) in input.into_iter().zip(senders.into_iter().cycle()) {
                tx.send(text).unwrap();
            }
            handlers
        }.into_iter()
        .map(|h| h.join().unwrap())
        .fold(HashMap::new(), fold_hashmap)
    })
}
