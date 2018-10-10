extern crate itertools;

use itertools::Itertools;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .map(|&i| (i..limit).step_by(i as usize))
        .kmerge()
        .dedup()
        .sum()
}
