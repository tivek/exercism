use std::collections::HashMap;

pub struct School {
    roster: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            roster: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roster
            .entry(grade)
            .or_default()
            .push(String::from(student));
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut out: Vec<u32> = self.roster.keys().cloned().collect();
        out.sort_unstable();
        out
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.roster.get(&grade).cloned().map(|mut names| {
            names.sort_unstable();
            names
        })
    }
}
