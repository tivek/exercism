extern crate itertools;

use itertools::Itertools;
use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    // This field is here to make the template compile and not to
    // complain about unused type lifetime parameter "'a". Once you start
    // solving the exercise, delete this field and the 'std::marker::PhantomData'
    // import.
    names: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.names.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut out = Vec::new();
        for codon in rna
            .chars()
            .chunks(3)
            .into_iter()
            .map(|chunk| chunk.collect::<String>())
        {
            match self.name_for(codon.as_str()) {
                None => return None,
                Some("stop codon") => break,
                Some(n) => out.push(n),
            }
        }

        Some(out)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        names: pairs.into_iter().collect(),
    }
}
