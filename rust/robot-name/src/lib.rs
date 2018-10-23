#[macro_use]
extern crate lazy_static;

extern crate rand;

use rand::Rng;
use std::collections::HashSet;
use std::sync::Mutex;

pub struct Robot {
    name: String,
}

fn get_unique_name() -> String {
    lazy_static! {
        static ref USED_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
    }
    let mut rng = rand::thread_rng();
    loop {
        let mut out = String::new();
        for _ in 0..2 {
            out.push(rng.gen_range(b'A', b'Z' + 1) as char);
        }
        for _ in 0..3 {
            out.push(rng.gen_range(b'0', b'9' + 1) as char);
        }
        let mut names = USED_NAMES.lock().unwrap();
        if !names.contains(&out) {
            names.insert(out.clone());
            return out;
        }
    }
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: get_unique_name(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        self.name = get_unique_name();
    }
}
